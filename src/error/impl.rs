use super::r#type::ServerError;
use crate::*;

impl StdError for ServerError {}

impl Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TcpBindError(data) => write!(f, "Tcp bind error{}{}", COLON_SPACE, data),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}
