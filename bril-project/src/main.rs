use std::{
    fs,
    io::{Read, stdin},
    path::PathBuf,
    str::FromStr,
};

use argh::FromArgs;
use bril_compiler::{cfg::construct_cfg, tdce::eliminate_dead_code};
use bril_rs::{Program, load_program_from_read};
use snafu::{ResultExt, Whatever};

#[derive(Debug, Default)]
struct Modules {
    pub c: bool,
    pub t: bool,
}

// Convert a string like "ctdegd" into active flags
impl FromStr for Modules {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut modules = Modules::default();
        for ch in s.chars() {
            match ch {
                'c' => modules.c = true,
                't' => modules.t = true,
                unknown => return Err(format!("Unknown module code: '{unknown}'")),
            }
        }
        Ok(modules)
    }
}

#[derive(FromArgs)]
/// Generates a CFG from Bril's canonical JSON representation
#[argh(help_triggers("-h", "--help", "help"))]
struct Opts {
    /// input Bril file: omit for stdin
    #[argh(positional)]
    input_path: Option<PathBuf>,
    /// active module letters, e.g., -m ct
    #[argh(option, short = 'm', default = "Modules::default()")]
    modules: Modules,
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
    let mut cfg = construct_cfg(program);

    if opts.modules.c {
        print!("{:?}", cfg);
    }

    if opts.modules.t {
        eliminate_dead_code(&mut cfg);
        let tdce_program: Program = cfg.into();
        print!("{}", tdce_program);
    }

    Ok(())
}
