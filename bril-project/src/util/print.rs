use std::io::{self, Write};

use crate::util::types::ProgramSuccessorMap;

pub fn print_graphviz<W: Write>(
    successor_map: &ProgramSuccessorMap,
    out: &mut W,
) -> io::Result<()> {
    for (func_name, func_body) in successor_map.iter() {
        writeln!(out, "diagraph {} {{", func_name)?;

        for label in func_body.keys() {
            writeln!(out, "    {};", label)?;
        }

        for (label, successors) in func_body {
            for successor in successors {
                writeln!(out, "    {} -> {};", label, successor)?;
            }
        }
    }

    writeln!(out, "}}")?;

    Ok(())
}
