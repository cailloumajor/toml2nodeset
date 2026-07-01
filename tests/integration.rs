use std::fs;
use std::path::Path;

use escargot::CargoBuild;

#[test]
fn cli() {
    let model_compiler_dir = Path::new(env!("CARGO_TARGET_TMPDIR"));
    let model_compiler_path = model_compiler_dir.join("Opc.Ua.ModelCompiler");
    let fake_model_compiler = CargoBuild::new()
        .package("fake-model-compiler")
        .run()
        .expect("building fake model compiler should not fail");
    fs::copy(fake_model_compiler.path(), model_compiler_path)
        .expect("copying built fake compiler should not fail");

    trycmd::TestCases::new()
        .env("PATH", model_compiler_dir.to_string_lossy())
        .case("tests/cmd/cli.toml");
}
