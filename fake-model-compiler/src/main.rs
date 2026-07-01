use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let get_flag_value = |arg: &str| {
        args.iter()
            .position(|a| a == arg)
            .map(|i| args[i + 1].as_str())
            .unwrap_or_else(|| panic!("getting `{arg}` flag value should not fail"))
    };

    // Create a file in the provided output directory to store sentinel data.
    let out_file_path = Path::new(get_flag_value("-o2")).join("model_compiler_out.txt");
    let mut out_file =
        File::create(out_file_path).expect("creating the output file should not fail");

    // Compiler mode.
    writeln!(&mut out_file, "mode: {}", args[1]).unwrap();

    // Model design file path.
    writeln!(&mut out_file, "design file: {}", get_flag_value("-d2")).unwrap();

    // Identifier file path.
    writeln!(&mut out_file, "identifier file: {}", get_flag_value("-cg")).unwrap();

    // Generated version.
    writeln!(&mut out_file, "version: {}", get_flag_value("-mv")).unwrap();
}
