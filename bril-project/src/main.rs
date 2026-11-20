use std::{
    fs,
    io::{Read, stdin},
    path::PathBuf,
};

use argh::FromArgs;
use bril_compiler::{
    cfg::construct_cfg, lvn::LocalValueNumbering, parser::Modules, tdce::DeadCodeElimination,
};
use bril_rs::{Program, load_program_from_read};
use snafu::{ResultExt, Whatever};

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
                        .whatever_context(format!("Failed to open file: {input_path_string}"))?,
                ),
            )
        } else {
            ("<stdin".to_owned(), Box::new(stdin()))
        }
    };

    let program = load_program_from_read(reader);
    let mut cfg = construct_cfg(program);

    // For testing purposes
    if opts.modules.dump_cfg {
        print!("{cfg:?}");
        return Ok(());
    }

    // Optimizations
    // Local Value Numbering
    if opts.modules.lvn {
        LocalValueNumbering::run(&mut cfg);
    }

    // Total Dead Code Elimination
    if opts.modules.tdce {
        DeadCodeElimination::run(&mut cfg);
    }

    // Dataflow analysis
    if let Some(analysis) = opts.modules.dfa {
        analysis.run(&cfg);
        return Ok(());
    }

    let optimized_program: Program = cfg.into();
    print!("{optimized_program}");

    Ok(())
}
