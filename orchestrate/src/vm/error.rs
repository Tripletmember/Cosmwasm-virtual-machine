use core::fmt::Display;
use cosmwasm_vm::{
    executor::ExecutorError,
    memory::{MemoryReadError, MemoryWriteError},
    system::{CosmwasmCodeId, SystemError},
};
use cosmwasm_vm_wasmi::WasmiVMError;
use wasmi::CanResume;

use super::Account;

#[derive(Debug)]
pub enum VmError {
    Interpreter(wasmi::Error),
    VMError(WasmiVMError),
    CodeNotFound(CosmwasmCodeId),
    ContractNotFound(Account),
    InvalidAddress,
    InvalidAccountFormat,
    NoCustomQuery,
    NoCustomMessage,
    Unsupported,
    OutOfGas,
    CryptoError,
    IteratorDoesNotExist,
    AlreadyInstantiated,
    CannotSerialize,
    UnknownIbcChannel,
}

impl From<wasmi::Error> for VmError {
    fn from(e: wasmi::Error) -> Self {
        Self::Interpreter(e)
    }
}

impl From<WasmiVMError> for VmError {
    fn from(e: WasmiVMError) -> Self {
        VmError::VMError(e)
    }
}

impl From<SystemError> for VmError {
    fn from(e: SystemError) -> Self {
        VmError::VMError(e.into())
    }
}

impl From<ExecutorError> for VmError {
    fn from(e: ExecutorError) -> Self {
        VmError::VMError(e.into())
    }
}

impl From<MemoryReadError> for VmError {
    fn from(e: MemoryReadError) -> Self {
        VmError::VMError(e.into())
    }
}

impl From<MemoryWriteError> for VmError {
    fn from(e: MemoryWriteError) -> Self {
        VmError::VMError(e.into())
    }
}

impl Display for VmError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl CanResume for VmError {
    fn can_resume(&self) -> bool {
        false
    }
}
