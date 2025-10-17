use crate::*;

/// Implementation of InnerContext methods.
impl InnerContext {
    /// Creates a new InnerContext with default values.
    ///
    /// # Returns
    ///
    /// - `InnerContext` - New context instance.
    pub fn new() -> Self {
        InnerContext {
            socket: None,
            request: Request::new(),
            response: Response::default(),
            socket_addr: None,
            data: HashMap::default(),
        }
    }
}

/// Implementation of Context methods.
///
/// Provides thread-safe operations on the UDP context.
impl Context {
    /// Creates a Context from an InnerContext.
    ///
    /// # Arguments
    ///
    /// - `InnerContext` - The inner context to wrap.
    ///
    /// # Returns
    ///
    /// - `Context` - New thread-safe context wrapper.
    pub(crate) fn from_inner_context(ctx: InnerContext) -> Self {
        Self(Arc::new(RwLock::new(ctx)))
    }

    /// Acquires a read lock on the inner context.
    ///
    /// # Returns
    ///
    /// - `RwLockReadContext` - Read guard for the inner context.
    pub async fn get_read_lock(&'_ self) -> RwLockReadContext<'_> {
        self.0.read().await
    }

    /// Acquires a write lock on the inner context.
    ///
    /// # Returns
    ///
    /// - `RwLockWriteContext` - Write guard for the inner context.
    pub async fn get_write_lock(&'_ self) -> RwLockWriteContext<'_> {
        self.0.write().await
    }

    /// Gets a clone of the inner context.
    ///
    /// # Returns
    ///
    /// - `InnerContext` - Clone of the inner context.
    pub async fn get(&self) -> InnerContext {
        self.get_read_lock().await.clone()
    }

    /// Gets the request data from the context.
    ///
    /// # Returns
    ///
    /// - `Request` - Clone of the request data.
    pub async fn get_request(&self) -> Request {
        self.get().await.request.clone()
    }

    /// Gets the response data from the context.
    ///
    /// # Returns
    ///
    /// - `Response` - Clone of the response data.
    pub async fn get_response(&self) -> Response {
        self.get().await.response.clone()
    }

    /// Gets the UDP socket from the context.
    ///
    /// # Returns
    ///
    /// - `OptionArcRwLockUdpSocket` - Clone of the socket if present.
    pub async fn get_socket(&self) -> OptionArcRwLockUdpSocket {
        self.get().await.socket.clone()
    }

    /// Gets the socket address from the context.
    ///
    /// # Returns
    ///
    /// - `OptionSocketAddr` - Clone of the socket address if present.
    pub async fn get_socket_addr(&self) -> OptionSocketAddr {
        self.get().await.socket_addr.clone()
    }

    /// Gets the socket address or default if not present.
    ///
    /// # Returns
    ///
    /// - `SocketAddr` - Socket address or default (0.0.0.0:0).
    pub async fn get_socket_addr_or_default(&self) -> SocketAddr {
        let socket_result: OptionArcRwLockUdpSocket = self.get_socket().await;
        if socket_result.is_none() {
            return DEFAULT_SOCKET_ADDR;
        }
        let socket_addr: SocketAddr = socket_result
            .unwrap()
            .get_read_lock()
            .await
            .peer_addr()
            .unwrap_or(DEFAULT_SOCKET_ADDR);
        socket_addr
    }

    /// Gets the socket address as a string.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - Socket address string if present.
    pub async fn get_socket_addr_string(&self) -> Option<String> {
        self.get_socket_addr().await.map(|data| data.to_string())
    }

    /// Gets the socket address or default as a string.
    ///
    /// # Returns
    ///
    /// - `String` - Socket address string or default.
    pub async fn get_socket_addr_or_default_string(&self) -> String {
        self.get_socket_addr_or_default().await.to_string()
    }

    /// Gets the socket host IP address.
    ///
    /// # Returns
    ///
    /// - `OptionSocketHost` - Host IP address if present.
    pub async fn get_socket_host(&self) -> OptionSocketHost {
        self.get_socket_addr()
            .await
            .map(|socket_addr: SocketAddr| socket_addr.ip())
    }

    /// Gets the socket port number.
    ///
    /// # Returns
    ///
    /// - `OptionSocketPort` - Port number if present.
    pub async fn get_socket_port(&self) -> OptionSocketPort {
        self.get_socket_addr()
            .await
            .map(|socket_addr: SocketAddr| socket_addr.port())
    }

    /// Sets the response data in the context.
    ///
    /// # Arguments
    ///
    /// - `T` - Data convertible to ResponseData.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for chaining.
    pub(super) async fn set_response<T: Into<ResponseData>>(&self, data: T) -> &Self {
        self.get_write_lock().await.response = Response::from(data);
        self
    }

    /// Sends response data through the socket.
    ///
    /// # Arguments
    ///
    /// - `T` - Data convertible to ResponseData.
    ///
    /// # Returns
    ///
    /// - `ResponseResult` - Result of the send operation.
    pub async fn send<T: Into<ResponseData>>(&self, data: T) -> ResponseResult {
        let response_result: ResponseResult = self
            .set_response(data)
            .await
            .get_response()
            .await
            .send(&self.get_socket().await, &self.get_socket_addr().await)
            .await;
        return response_result;
    }

    /// Sets a value in the context data storage.
    ///
    /// # Arguments
    ///
    /// - `&str` - Key for the value.
    /// - `&T` - Value to store (must be Any + Send + Sync + Clone).
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for chaining.
    pub async fn set_data_value<T: Any + Send + Sync + Clone>(
        &self,
        key: &str,
        value: &T,
    ) -> &Self {
        self.get_write_lock()
            .await
            .data
            .insert(key.to_owned(), Arc::new(value.clone()));
        self
    }

    /// Gets a value from the context data storage.
    ///
    /// # Arguments
    ///
    /// - `&str` - Key for the value.
    ///
    /// # Returns
    ///
    /// - `Option<T>` - Retrieved value if present and of correct type.
    pub async fn get_data_value<T: Any + Send + Sync + Clone>(&self, key: &str) -> Option<T> {
        self.get_read_lock()
            .await
            .data
            .get(key)
            .and_then(|arc| arc.downcast_ref::<T>())
            .cloned()
    }

    /// Removes a value from the context data storage.
    ///
    /// # Arguments
    ///
    /// - `&str` - Key for the value to remove.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for chaining.
    pub async fn remove_data_value(&self, key: &str) -> &Self {
        self.get_write_lock().await.data.remove(key);
        self
    }

    /// Clears all data from the context data storage.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for chaining.
    pub async fn clear_data(&self) -> &Self {
        self.get_write_lock().await.data.clear();
        self
    }
}
