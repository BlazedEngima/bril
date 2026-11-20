use std::fmt::{Display, Formatter, Result};

use crate::types::FunctionCFG;

pub struct DataFlowResult<D: Display> {
    pub in_facts: Vec<D>,
    pub out_facts: Vec<D>,
}

impl<D: Display> DataFlowResult<D> {
    pub fn print_with_header_names(&self, func_cfg: &FunctionCFG) {
        for (idx, input) in self.in_facts.iter().enumerate() {
            let output = &self.out_facts[idx];
            if let Some(label) = func_cfg.get_label(idx) {
                let name = &label.name;
                println!("{name}:");
            } else {
                println!("BB{idx}:");
            }

            println!("  in:  {input}");
            println!("  out: {output}");
        }
    }
}

impl<D: Display> Display for DataFlowResult<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for (idx, input) in self.in_facts.iter().enumerate() {
            let output = &self.out_facts[idx];

            writeln!(f, "BB{idx}:")?;
            writeln!(f, "  in:  {input}")?;
            writeln!(f, "  out: {output}")?;
        }

        Ok(())
    }
}
