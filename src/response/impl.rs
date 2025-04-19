use crate::*;

impl Default for Response {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl Response {
    pub fn from<T: Into<ResponseData>>(data: T) -> Self {
        Self(data.into())
    }

    pub fn get_response_data(&self) -> &ResponseData {
        &self.0
    }

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
