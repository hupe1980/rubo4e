//! E05-S22 to S24 — versioned module coverage assertions.
//!
//! These tests run under `--features versioned`.

#[cfg(feature = "versioned")]
mod coverage {
    use std::path::PathBuf;

    fn workspace_root() -> PathBuf {
        // CARGO_MANIFEST_DIR is the workspace root for the `rubo4e` crate.
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    fn count_schemas(schema_version: &str, category: &str) -> Option<usize> {
        let dir = workspace_root()
            .join("generator/schemas")
            .join(schema_version)
            .join(category);
        let entries = match std::fs::read_dir(&dir) {
            Ok(e) => e,
            Err(_) => {
                eprintln!(
                    "SKIP: schema dir not found (run `just download-schemas {schema_version}`): {}",
                    dir.display()
                );
                return None;
            }
        };
        Some(
            entries
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().map(|x| x == "json").unwrap_or(false))
                .count(),
        )
    }

    fn count_generated(schema_version_dir: &str, category: &str) -> Option<usize> {
        // e.g. "v202501" -> src/generated/v202501/
        let dir = workspace_root()
            .join("src/generated")
            .join(schema_version_dir)
            .join(category);
        let entries = match std::fs::read_dir(&dir) {
            Ok(e) => e,
            Err(_) => return None,
        };
        Some(
            entries
                .filter_map(|e| e.ok())
                .filter(|e| {
                    let name = e.file_name();
                    let n = name.to_string_lossy();
                    n.ends_with(".rs") && n != "mod.rs"
                })
                .count(),
        )
    }

    /// E05-S22 / E05-S24: v202501 type counts meet minimum CONCEPT.md thresholds.
    #[test]
    fn v202501_schema_counts() {
        let Some(bo_count) = count_schemas("v202501.0.0", "bo") else {
            return;
        };
        let Some(com_count) = count_schemas("v202501.0.0", "com") else {
            return;
        };
        let Some(enum_count) = count_schemas("v202501.0.0", "enum") else {
            return;
        };

        // Verify counts haven't dropped below a sanity floor.
        assert!(bo_count >= 35, "expected ≥35 BO schemas, got {bo_count}");
        assert!(com_count >= 60, "expected ≥60 COM schemas, got {com_count}");
        assert!(
            enum_count >= 70,
            "expected ≥70 enum schemas, got {enum_count}"
        );

        // Verify generated Rust files match the schema counts.
        if let Some(gen_bo) = count_generated("v202501", "bo") {
            assert_eq!(gen_bo, bo_count, "v202501 generated BO count ({gen_bo}) != schema count ({bo_count}); run `just generate`");
        }
        if let Some(gen_com) = count_generated("v202501", "com") {
            assert_eq!(gen_com, com_count, "v202501 generated COM count ({gen_com}) != schema count ({com_count}); run `just generate`");
        }
        if let Some(gen_enum) = count_generated("v202501", "enum") {
            assert_eq!(gen_enum, enum_count, "v202501 generated enum count ({gen_enum}) != schema count ({enum_count}); run `just generate`");
        }
    }

    /// E05-S22: `rubo4e::v202501` is accessible as the current version module.
    /// Compile-time test — if this doesn't compile, the feature gate is broken.
    #[test]
    fn v202501_is_current_version() {
        // `rubo4e::v202501` is the current latest stable version
        let _: rubo4e::v202501::Vertrag = rubo4e::v202501::Vertrag::default();
    }

    /// E05-S25: No `v202402` schema directory exists.
    #[test]
    fn no_v202402_schema_directory() {
        let schemas_dir = workspace_root().join("generator/schemas");
        let entries = match std::fs::read_dir(&schemas_dir) {
            Ok(e) => e,
            Err(_) => {
                eprintln!("SKIP: generator/schemas/ not present — run `just download-schemas`");
                return;
            }
        };
        let bad_dirs: Vec<_> = entries
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.file_name()
                    .to_str()
                    .map(|n| n.starts_with("v202402"))
                    .unwrap_or(false)
            })
            .collect();

        assert!(
            bad_dirs.is_empty(),
            "v202402 schema directory must not exist: {bad_dirs:?}"
        );
    }

    /// Verifies no source file in src/ contains a v202402 reference.
    #[test]
    fn no_v202402_in_source() {
        let src_dir = workspace_root().join("src");
        let mut violations: Vec<String> = Vec::new();
        scan_dir_for_pattern(&src_dir, "v202402", &mut violations);
        assert!(
            violations.is_empty(),
            "Found v202402 references in source:\n{}",
            violations.join("\n")
        );
    }

    fn scan_dir_for_pattern(dir: &std::path::Path, pattern: &str, out: &mut Vec<String>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_dir() {
                scan_dir_for_pattern(&path, pattern, out);
            } else if path.extension().map(|e| e == "rs").unwrap_or(false) {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if content.contains(pattern) {
                        out.push(path.display().to_string());
                    }
                }
            }
        }
    }
}
