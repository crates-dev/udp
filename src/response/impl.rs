use crate::*;

/// Default implementation for Response.
impl Default for Response {
    /// Creates a default empty response.
    ///
    /// # Returns
    ///
    /// - `Response` - New response with empty data.
    fn default() -> Self {
        Self(Vec::new())
    }
}

/// Implementation of Response methods.
///
/// Provides conversion and sending capabilities for UDP responses.
impl Response {
    /// Creates a Response from convertible data.
    ///
    /// # Arguments
    ///
    /// - `T` - Data convertible to ResponseData.
    ///
    /// # Returns
    ///
    /// - `Response` - New response containing the data.
    pub fn from<T: Into<ResponseData>>(data: T) -> Self {
        Self(data.into())
    }

    /// Gets the underlying response data.
    ///
    /// # Returns
    ///
    /// - `&ResponseData` - Reference to the response data.
    pub fn get_response_data(&self) -> &ResponseData {
        &self.0
    }

    /// Sends the response through the specified socket.
    ///
    /// # Arguments
    ///
    /// - `&OptionArcRwLockUdpSocket` - Optional socket reference.
    /// - `&OptionSocketAddr` - Optional target address.
    ///
    /// # Returns
    ///
    /// - `ResponseResult` - Result of the send operation.
    pub async fn send(
        &self,
        socket_opt: &OptionArcRwLockUdpSocket,
        addr_opt: &OptionSocketAddr,
    ) -> ResponseResult {
        if let Some(socket_lock) = socket_opt {
            let socket = socket_lock.get_read_lock().await;
            if let Some(addr) = addr_opt {
                let response_data: &ResponseData = self.get_response_data();
                socket
                    .send_to(response_data, &addr)
                    .await
                    .map_err(|e| response::error::ResponseError::ResponseError(e.to_string()))?;
                return Ok(());
            }
        }
        Err(response::error::ResponseError::Unknown)
    }
}
