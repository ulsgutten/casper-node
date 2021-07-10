//! Errors that the contract runtime component may raise.

use casper_execution_engine::{
    core::engine_state::{Error as EngineStateError, StepError},
    shared::TypeMismatch,
    storage::error::lmdb::Error as StorageLmdbError,
};
use casper_types::{bytesrepr, Key};

use crate::{crypto::hash::Digest, types::error::BlockCreationError};

/// Error returned from mis-configuring the contract runtime component.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// Error initializing the LMDB environment.
    #[error("failed to initialize LMDB environment for contract runtime: {0}")]
    Lmdb(#[from] StorageLmdbError),
    /// Error initializing metrics.
    #[error("failed to initialize metrics for contract runtime: {0}")]
    Prometheus(#[from] prometheus::Error),
}

/// An error raised by a contract runtime variant.
#[derive(Debug, thiserror::Error)]
pub enum BlockExecutionError {
    /// Currently the contract runtime can only execute one commit at a time, so we cannot handle
    /// more than one execution result.
    #[error("More than one execution result")]
    MoreThanOneExecutionResult,
    /// A core error thrown by the execution engine.
    #[error(transparent)]
    EngineStateError(#[from] EngineStateError),
    /// An error that occurred when trying to run the auction contract.
    #[error(transparent)]
    StepError(#[from] StepError),
    /// An error that occurred when committing execution results to the trie.
    #[error(transparent)]
    CommitError(#[from] CommitError),
    /// An error that occurred while creating a block.
    #[error(transparent)]
    BlockCreationError(#[from] BlockCreationError),
}

/// An error emitted by the execution engine on commit
#[derive(Debug, thiserror::Error)]
pub enum CommitError {
    #[error(transparent)]
    EngineStateError(#[from] EngineStateError),

    #[error("Root not found: {0:?}")]
    RootNotFound(Digest),
    #[error("Key not found: {0}")]
    KeyNotFound(Key),
    #[error("Type mismatch: {0}")]
    TypeMismatch(TypeMismatch),
    #[error("Serialization: {0}")]
    Serialization(bytesrepr::Error),
}
