use crate::*;

/// Default implementation for Response.
impl Default for Response {
    /// Creates a default empty response.
    ///
    /// # Returns
    ///
    /// - `Response` - New response with empty data.
    #[inline(always)]
    fn default() -> Self {
        Self { data: Vec::new() }
    }
}

/// Implementation of Response methods.
impl Response {
    /// Creates a Response from convertible data.
    ///
    /// # Arguments
    ///
    /// - `Into<ResponseData>` - Data convertible to ResponseData.
    ///
    /// # Returns
    ///
    /// - `Response` - New response containing the data.
    #[inline(always)]
    pub fn from<T>(data: T) -> Self
    where
        T: Into<ResponseData>,
    {
        Self { data: data.into() }
    }

    /// Gets the underlying response data.
    ///
    /// # Returns
    ///
    /// - `&ResponseData` - Reference to the response data.
    pub fn get_data(&self) -> &ResponseData {
        &self.data
    }

    /// Sends the response through the specified socket.
    ///
    /// # Arguments
    ///
    /// - `&Option<ArcRwLockUdpSocket>` - Optional socket reference.
    /// - `&Option<SocketAddr>` - Optional target address.
    ///
    /// # Returns
    ///
    /// - `ResponseResult` - Result of the send operation.
    pub async fn send(
        &self,
        socket_opt: &Option<ArcRwLockUdpSocket>,
        addr_opt: &Option<SocketAddr>,
    ) -> ResponseResult {
        if let Some(socket_lock) = socket_opt {
            let socket: tokio::sync::RwLockReadGuard<'_, UdpSocket> =
                socket_lock.get_read_lock().await;
            if let Some(addr) = addr_opt {
                socket
                    .send_to(self.get_data(), addr)
                    .await
                    .map_err(|e| ResponseError::SendError(e.to_string()))?;
                return Ok(());
            }
            return Err(ResponseError::AddressNotAvailable);
        }
        Err(ResponseError::SocketNotAvailable)
    }
}
