
use super::*;


#[derive(Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
pub enum PoolStatusBitFlag {
    Enable,
    Disable,
}


