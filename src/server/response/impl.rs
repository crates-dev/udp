use crate::*;

impl Default for Response {
    #[inline]
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl Response {
    #[inline]
    pub fn from<T: Into<ResponseData>>(data: T) -> Self {
        Self(data.into())
    }

    #[inline]
    pub fn get_response_data(&self) -> &ResponseData {
        &self.0
    }

    #[inline]
    pub async fn send(
        &self,
        socket_opt: &OptionArcRwLockUdpSocket,
        addr_opt: &OptionSocketAddr,
    ) -> ResponseResult {
        if let Some(socket_lock) = socket_opt {
            let socket = socket_lock.get_read_lock().await;
            if let Some(addr) = addr_opt {
                let response_data: &ResponseData = self.get_response_data();
                return socket
                    .send_to(response_data, &addr)
                    .await
                    .and_then(|_| Ok(response_data.clone()))
                    .map_err(|e| server::response::error::Error::ResponseError(e.to_string()));
            }
        }
        Err(server::response::error::Error::Unknown)
    }
}
