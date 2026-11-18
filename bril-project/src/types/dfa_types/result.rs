use ahash::HashMap;

use crate::types::BlockId;

pub struct DataFlowResult<D> {
    pub in_facts: HashMap<BlockId, D>,
    pub out_facts: HashMap<BlockId, D>,
}
