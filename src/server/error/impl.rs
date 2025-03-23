use super::r#type::Error;
use crate::*;

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TcpBindError(data) => write!(f, "Tcp bind error{}{}", COLON_SPACE, data),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}
