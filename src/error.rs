use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("tarantool not found")]
    TarantoolNotFound,
    #[error("the child process terminated with an error")]
    ChildProcessTerminated,
}
