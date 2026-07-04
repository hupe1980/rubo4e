use bo4e_generator::{emitter, parser};

use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use clap::Parser;

// ─── CLI ──────────────────────────────────────────────────────────────────────

/// BO4E Rust code generator.
///
/// Reads JSON Schema files from `generator/schemas/<schema-version>/`
/// and writes Rust source files to `src/generated/<series>/`.
///
/// The *series* is the first 7 characters of the schema version tag,
/// e.g. `v202501.0.0` → `v202501`.
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// BO4E schema version tag, e.g. `v202501.0.0`
    #[arg(long, short = 'v')]
    schema_version: String,

    /// Root of the repository. Defaults to the current directory.
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,
}

// ─── Main ─────────────────────────────────────────────────────────────────────

fn main() -> Result<()> {
    let args = Args::parse();

    // Derive the series ("v202501") and paths.
    let series = series_from_version(&args.schema_version)?;
    let schema_dir = schema_dir_for(&args.repo_root, &args.schema_version);
    let out_root = args.repo_root.join("src").join("generated").join(&series);

    eprintln!("Schema version : {}", args.schema_version);
    eprintln!("Schema dir     : {}", schema_dir.display());
    eprintln!("Output dir     : {}", out_root.display());

    let all_nodes = parse_schema_nodes(&schema_dir, &args.schema_version)?;

    if all_nodes.is_empty() {
        bail!(
            "no schema files found under {}\n\
             Expected subdirectories: bo/, com/, enum/",
            schema_dir.display()
        );
    }

    eprintln!("Parsed {} schemas.", all_nodes.len());

    // Emit Rust files.
    std::fs::create_dir_all(&out_root)
        .with_context(|| format!("creating output dir: {}", out_root.display()))?;

    for node in &all_nodes {
        let (filename, source) = emitter::emit_node(node, &args.schema_version)?;
        let dest = out_root.join(&filename);
        write_if_changed(&dest, &source)?
    }

    // Write mod.rs.
    let mod_rs = emitter::emit_mod_rs(&all_nodes, &args.schema_version)?;
    write_if_changed(&out_root.join("mod.rs"), &mod_rs)?;

    eprintln!(
        "Done. {} files written to {}",
        all_nodes.len() + 1,
        out_root.display()
    );

    Ok(())
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// Extracts the 7-character series prefix from a full version tag.
///
/// `"v202501.0.0"` → `"v202501"`
fn series_from_version(version: &str) -> Result<String> {
    if version.len() < 7 {
        bail!("schema_version must be at least 7 characters (e.g. v202501.0.0), got: {version}");
    }
    Ok(version[..7].to_owned())
}

fn schema_dir_for(repo_root: &Path, version: &str) -> PathBuf {
    repo_root.join("generator").join("schemas").join(version)
}

/// Parses BO4E schema JSON files for one concrete version directory.
///
/// Includes category subdirectories (`bo`, `com`, `enum`) and root-level JSON
/// files (treated as COM types, e.g. `ZusatzAttribut.json`).
fn parse_schema_nodes(
    schema_dir: &Path,
    version: &str,
) -> Result<Vec<bo4e_generator::ast::SchemaNode>> {
    if !schema_dir.exists() {
        bail!(
            "schema directory does not exist: {}\n\
             Download it with:\n  \
             just download-schemas {}",
            schema_dir.display(),
            version
        );
    }

    // Parse each category subdirectory (bo/, com/, enum/) separately so the
    // category is available for BO vs COM classification.
    let mut nodes = Vec::new();
    for category in ["bo", "com", "enum"] {
        let cat_dir = schema_dir.join(category);
        if cat_dir.exists() {
            let parsed = parser::parse_dir(&cat_dir).with_context(|| {
                format!("parsing {category} schemas in {}", schema_dir.display())
            })?;
            nodes.extend(parsed);
        }
    }

    // Also parse root-level schema files (e.g. ZusatzAttribut.json).
    // These are treated as COM types since they have no category directory.
    let root_schemas: Vec<_> = std::fs::read_dir(schema_dir)
        .with_context(|| format!("reading schema dir: {}", schema_dir.display()))?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|x| x == "json").unwrap_or(false))
        .collect();
    for entry in root_schemas {
        let node = parser::parse_file_as_com(&entry.path())
            .with_context(|| format!("parsing {}", entry.path().display()))?;
        if let Some(n) = node {
            nodes.push(n);
        }
    }

    if nodes.is_empty() {
        bail!(
            "no schema files found under {}\n\
             Expected subdirectories: bo/, com/, enum/",
            schema_dir.display()
        );
    }

    Ok(nodes)
}

/// Writes `content` to `path` only if the existing file has different content,
/// to avoid unnecessary filesystem writes and mtime changes.
fn write_if_changed(path: &Path, content: &str) -> Result<()> {
    if path.exists() {
        let existing =
            std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        if existing == content {
            return Ok(());
        }
    }
    std::fs::write(path, content).with_context(|| format!("writing {}", path.display()))?;
    eprintln!("  wrote {}", path.display());
    Ok(())
}
