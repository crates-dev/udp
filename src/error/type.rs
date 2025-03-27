#[derive(Debug)]
pub enum ServerError {
    TcpBindError(String),
    Unknown,
}
