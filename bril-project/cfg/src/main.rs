use std::{
    fs,
    io::{Read, stdin, stdout},
    path::PathBuf,
};

use argh::FromArgs;
use bril_rs::load_program_from_read;
use cfg::core::{get_cfg, get_successor_map, print_graphviz};
use snafu::{ResultExt, Whatever};

#[derive(FromArgs)]
/// Generates a CFG from Bril's canonical JSON representation
#[argh(help_triggers("-h", "--help", "help"))]
struct Opts {
    /// input Bril file: omit for stdin
    #[argh(positional)]
    input_path: Option<PathBuf>,
}

#[snafu::report]
fn main() -> Result<(), Whatever> {
    let opts = argh::from_env::<Opts>();

    let (_, reader): (String, Box<dyn Read>) = {
        if let Some(input_path) = opts.input_path {
            let input_path_string = input_path.to_string_lossy().to_string();
            (
                input_path_string.clone(),
                Box::new(
                    fs::File::open(&input_path)
                        .whatever_context(format!("Failed to open file: {}", input_path_string))?,
                ),
            )
        } else {
            ("<stdin".to_owned(), Box::new(stdin()))
        }
    };

    let program = load_program_from_read(reader);
    let cfg = get_cfg(program);
    let successor_map = get_successor_map(&cfg);

    print_graphviz(&successor_map, &mut stdout()).whatever_context("Unable to write to stdout")?;

    Ok(())
}
