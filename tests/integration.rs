use std::path::Path;

#[test]
fn cli() {
    const TOOLS_FILE: &str = "dotnet-tools.json";

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    std::fs::copy(
        manifest_dir.join(TOOLS_FILE),
        manifest_dir.join("tests/cmd/cli.in").join(TOOLS_FILE),
    )
    .expect("copying the tools manifest should not fail");

    trycmd::TestCases::new().case("tests/cmd/cli.toml");
}
