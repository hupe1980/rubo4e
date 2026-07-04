/// A struct with a required field (no `#[builder(default)]`).
/// Omitting it when building must be a compile error.
#[derive(typed_builder::TypedBuilder)]
struct RequiredFields {
    required: String,
    #[builder(default)]
    optional: Option<u32>,
}

fn main() {
    // `required` is omitted — this must not compile.
    let _ = RequiredFields::builder().optional(Some(42)).build();
}
