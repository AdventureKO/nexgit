pub type Result<T> = std::result::Result<T, CoreError>;

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("{0}")]
    Message(String),

    #[error("not implemented yet: {0}")]
    NotImplemented(&'static str),
}
