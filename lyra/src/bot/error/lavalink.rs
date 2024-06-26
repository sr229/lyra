use thiserror::Error;

#[derive(Error, Debug)]
#[error("player does not yet exist")]
pub struct NoPlayerError;

#[derive(Error, Debug)]
#[error("processing lavalink event failed: {:?}", .0)]
pub enum ProcessError {
    Lavalink(#[from] lavalink_rs::error::LavalinkError),
}

pub type ProcessResult = Result<(), ProcessError>;
