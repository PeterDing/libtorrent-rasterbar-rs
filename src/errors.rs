use thiserror::Error;

pub type LTResult<T> = Result<T, LTError>;

#[derive(Error, Debug)]
pub enum LTError {
    #[error("Failed to create session: {0}")]
    FailedToCreateSession(String),

    #[error("Failed to add torrent: {0}")]
    FailedToAddTorrent(String),

    #[error("Failed to add magnet: {0}")]
    FailedToAddMagnet(String),
}
