use crate::tx_execution::BlockchainVMRef;
use multiversx_chain_vm_executor::Executor;
use std::{fmt::Debug, ops::Deref};

use super::{BlockchainStateRef, FailingExecutor};

pub struct BlockchainMock {
    pub vm: BlockchainVMRef,
    pub state: BlockchainStateRef,
}

impl BlockchainMock {
    pub fn new(executor: Box<dyn Executor + Send + Sync>) -> Self {
        BlockchainMock {
            vm: BlockchainVMRef::new(executor),
            state: Default::default(),
        }
    }
}

impl Default for BlockchainMock {
    fn default() -> Self {
        Self::new(Box::new(FailingExecutor))
    }
}

impl Debug for BlockchainMock {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BlockchainMock")
            .field("state", self.state.deref())
            .finish()
    }
}
