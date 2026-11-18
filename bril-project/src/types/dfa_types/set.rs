use std::{
    fmt::{Debug, Display, Formatter, Result},
    ops::{Deref, DerefMut},
};

use indexmap::IndexSet;

use crate::types::DataFlowElement;

#[derive(Clone, PartialEq, Eq, Default)]
pub struct DataFlowSet<T: DataFlowElement>(pub IndexSet<T>);

impl<T: DataFlowElement> Deref for DataFlowSet<T> {
    type Target = IndexSet<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: DataFlowElement> DerefMut for DataFlowSet<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: DataFlowElement> Display for DataFlowSet<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.0.is_empty() {
            return write!(f, "∅");
        }

        for (i, item) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }

            write!(f, "{item}")?;
        }

        Ok(())
    }
}

impl<T: DataFlowElement> Debug for DataFlowSet<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.0.is_empty() {
            return write!(f, "∅");
        }

        for (i, item) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }

            write!(f, "{item:?}")?;
        }

        Ok(())
    }
}
