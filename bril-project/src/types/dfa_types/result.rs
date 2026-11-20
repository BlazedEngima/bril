use std::fmt::{Display, Formatter, Result};

pub struct DataFlowResult<D: Display> {
    pub in_facts: Vec<D>,
    pub out_facts: Vec<D>,
}

impl<D: Display> Display for DataFlowResult<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for (idx, input) in self.in_facts.iter().enumerate() {
            let output = &self.out_facts[idx];

            writeln!(f, "b{idx}:")?;
            writeln!(f, "  in:  {input}")?;
            writeln!(f, "  out: {output}")?;
        }

        Ok(())
    }
}
