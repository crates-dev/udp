use crate::*;

/// Provides a default implementation for ContextData.
impl Default for ContextData {
    /// Creates a default ContextData with empty values.
    ///
    /// # Returns
    ///
    /// - `Self` - A new instance with default values.
    #[inline(always)]
    fn default() -> Self {
        Self {
            aborted: false,
            socket: None,
            request: Request::new(),
            response: Response::default(),
            client_addr: None,
            attributes: HashMap::new(),
        }
    }
}

/// Provides a default implementation for Context.
impl Default for Context {
    /// Creates a default Context wrapping default ContextData.
    ///
    /// # Returns
    ///
    /// - `Self` - A new instance with default values.
    #[inline(always)]
    fn default() -> Self {
        Self(arc_rwlock(ContextData::default()))
    }
}

/// Implementation of methods for ContextData.
impl ContextData {
    /// Creates a new ContextData with the given socket, request, and client address.
    ///
    /// # Arguments
    ///
    /// - `ArcRwLockUdpSocket` - The network socket.
    /// - `Request` - The request data.
    /// - `SocketAddr` - The client's socket address.
    ///
    /// # Returns
    ///
    /// - `Self` - A new ContextData instance.
    pub fn new(socket: ArcRwLockUdpSocket, request: Request, client_addr: SocketAddr) -> Self {
        Self {
            aborted: false,
            socket: Some(socket),
            request,
            response: Response::default(),
            client_addr: Some(client_addr),
            attributes: HashMap::new(),
        }
    }

    /// Gets the aborted flag.
    ///
    /// # Returns
    ///
    /// - `bool` - The aborted flag.
    pub fn get_aborted(&self) -> bool {
        self.aborted
    }

    /// Sets the aborted flag.
    ///
    /// # Arguments
    ///
    /// - `bool` - The new value for the aborted flag.
    pub fn set_aborted(&mut self, aborted: bool) {
        self.aborted = aborted;
    }

    /// Gets the socket.
    ///
    /// # Returns
    ///
    /// - `Option<ArcRwLockUdpSocket>` - The socket if present.
    pub fn get_socket(&self) -> Option<ArcRwLockUdpSocket> {
        self.socket.clone()
    }

    /// Gets the request.
    ///
    /// # Returns
    ///
    /// - `&Request` - Reference to the request.
    pub fn get_request(&self) -> &Request {
        &self.request
    }

    /// Gets the response.
    ///
    /// # Returns
    ///
    /// - `&Response` - Reference to the response.
    pub fn get_response(&self) -> &Response {
        &self.response
    }

    /// Gets the response mutably.
    ///
    /// # Returns
    ///
    /// - `&mut Response` - Mutable reference to the response.
    pub fn get_response_mut(&mut self) -> &mut Response {
        &mut self.response
    }

    /// Gets the client address.
    ///
    /// # Returns
    ///
    /// - `Option<SocketAddr>` - The client address if present.
    pub fn get_client_addr(&self) -> Option<SocketAddr> {
        self.client_addr
    }

    /// Gets the attributes.
    ///
    /// # Returns
    ///
    /// - `&ThreadSafeAttributeStore` - Reference to the attributes.
    pub fn get_attributes(&self) -> &ThreadSafeAttributeStore {
        &self.attributes
    }

    /// Gets the attributes mutably.
    ///
    /// # Returns
    ///
    /// - `&mut ThreadSafeAttributeStore` - Mutable reference to the attributes.
    pub fn get_attributes_mut(&mut self) -> &mut ThreadSafeAttributeStore {
        &mut self.attributes
    }
}

/// Implementation of methods for Context.
impl Context {
    /// Creates a new Context with the given socket, request, and client address.
    ///
    /// # Arguments
    ///
    /// - `socket` - The network socket.
    /// - `request` - The request data.
    /// - `client_addr` - The client's socket address.
    ///
    /// # Returns
    ///
    /// - `Self` - A new Context instance.
    pub fn new(socket: &ArcRwLockUdpSocket, request: &Request, client_addr: SocketAddr) -> Self {
        Self(arc_rwlock(ContextData::new(
            socket.clone(),
            request.clone(),
            client_addr,
        )))
    }

    /// Acquires a read lock on the inner context data.
    ///
    /// # Returns
    ///
    /// - `RwLockReadGuard<ContextData>` - The read guard for the inner context.
    async fn read(&self) -> RwLockReadGuard<'_, ContextData> {
        self.0.read().await
    }

    /// Acquires a write lock on the inner context data.
    ///
    /// # Returns
    ///
    /// - `RwLockWriteGuard<ContextData>` - The write guard for the inner context.
    async fn write(&self) -> RwLockWriteGuard<'_, ContextData> {
        self.0.write().await
    }

    /// Checks if the context has been marked as aborted.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the context is aborted, otherwise false.
    pub async fn get_aborted(&self) -> bool {
        self.read().await.get_aborted()
    }

    /// Sets the aborted flag for the context.
    ///
    /// # Arguments
    ///
    /// - `aborted` - The aborted state to set.
    pub async fn set_aborted(&self, aborted: bool) {
        self.write().await.set_aborted(aborted);
    }

    /// Marks the context as aborted.
    pub async fn aborted(&self) {
        self.set_aborted(true).await;
    }

    /// Cancels the aborted state of the context.
    pub async fn cancel_aborted(&self) {
        self.set_aborted(false).await;
    }

    /// Retrieves the underlying network socket, if available.
    ///
    /// # Returns
    ///
    /// - `Option<ArcRwLockUdpSocket>` - The thread-safe, shareable network socket if it exists.
    pub async fn try_get_socket(&self) -> Option<ArcRwLockUdpSocket> {
        self.read().await.get_socket()
    }

    /// Retrieves the underlying network socket.
    ///
    /// # Returns
    ///
    /// - `ArcRwLockUdpSocket` - The thread-safe, shareable network socket.
    ///
    /// # Panics
    ///
    /// - If the network socket is not found.
    pub async fn get_socket(&self) -> ArcRwLockUdpSocket {
        self.try_get_socket().await.unwrap()
    }

    /// Retrieves the current request.
    ///
    /// # Returns
    ///
    /// - `Request` - A clone of the current request.
    pub async fn get_request(&self) -> Request {
        self.read().await.get_request().clone()
    }

    /// Retrieves the current response.
    ///
    /// # Returns
    ///
    /// - `Response` - A clone of the current response.
    pub async fn get_response(&self) -> Response {
        self.read().await.get_response().clone()
    }

    /// Retrieves the client address, if available.
    ///
    /// # Returns
    ///
    /// - `Option<SocketAddr>` - The client address if present.
    pub async fn try_get_client_addr(&self) -> Option<SocketAddr> {
        self.read().await.get_client_addr()
    }

    /// Retrieves the client address.
    ///
    /// # Returns
    ///
    /// - `SocketAddr` - The client address.
    ///
    /// # Panics
    ///
    /// - If the client address is not found.
    pub async fn get_client_addr(&self) -> SocketAddr {
        self.try_get_client_addr().await.unwrap()
    }

    /// Retrieves the client address as a string.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - The string representation of the client address if available.
    pub async fn try_get_client_addr_string(&self) -> Option<String> {
        self.try_get_client_addr()
            .await
            .map(|addr| addr.to_string())
    }

    /// Retrieves the client host IP address.
    ///
    /// # Returns
    ///
    /// - `Option<IpAddr>` - The client IP address if available.
    pub async fn try_get_client_host(&self) -> Option<IpAddr> {
        self.try_get_client_addr().await.map(|addr| addr.ip())
    }

    /// Retrieves the client port number.
    ///
    /// # Returns
    ///
    /// - `Option<u16>` - The client port number if available.
    pub async fn try_get_client_port(&self) -> Option<u16> {
        self.try_get_client_addr().await.map(|addr| addr.port())
    }

    /// Sets an attribute in the context.
    ///
    /// # Arguments
    ///
    /// - `K` - The key of the attribute to set.
    /// - `V` - The value of the attribute.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to the modified context.
    pub async fn set_attribute<K, V>(&self, key: K, value: V) -> &Self
    where
        K: AsRef<str>,
        V: AnySendSyncClone,
    {
        self.write()
            .await
            .get_attributes_mut()
            .insert(key.as_ref().to_owned(), Arc::new(value));
        self
    }

    /// Attempts to retrieve a specific attribute by its key.
    ///
    /// # Arguments
    ///
    /// - `K` - The key of the attribute to retrieve.
    ///
    /// # Returns
    ///
    /// - `Option<V>` - The attribute value if it exists and can be cast to the specified type.
    pub async fn try_get_attribute<V, K>(&self, key: K) -> Option<V>
    where
        V: AnySendSyncClone,
        K: AsRef<str>,
    {
        self.read()
            .await
            .get_attributes()
            .get(key.as_ref())
            .and_then(|arc| arc.downcast_ref::<V>())
            .cloned()
    }

    /// Retrieves a specific attribute by its key.
    ///
    /// # Arguments
    ///
    /// - `K` - The key of the attribute to retrieve.
    ///
    /// # Returns
    ///
    /// - `V` - The attribute value.
    ///
    /// # Panics
    ///
    /// - If the attribute is not found.
    pub async fn get_attribute<V, K>(&self, key: K) -> V
    where
        V: AnySendSyncClone,
        K: AsRef<str>,
    {
        self.try_get_attribute(key).await.unwrap()
    }

    /// Removes an attribute from the context.
    ///
    /// # Arguments
    ///
    /// - `K` - The key of the attribute to remove.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to the modified context.
    pub async fn remove_attribute<K>(&self, key: K) -> &Self
    where
        K: AsRef<str>,
    {
        self.write().await.get_attributes_mut().remove(key.as_ref());
        self
    }

    /// Clears all attributes from the context.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to the modified context.
    pub async fn clear_attributes(&self) -> &Self {
        self.write().await.get_attributes_mut().clear();
        self
    }

    /// Sends a response to the client.
    ///
    /// # Arguments
    ///
    /// - `Into<ResponseData>` - The response data.
    ///
    /// # Returns
    ///
    /// - `ResponseResult` - Result of the send operation.
    pub async fn send<T>(&self, data: T) -> ResponseResult
    where
        T: Into<ResponseData>,
    {
        let socket_opt: Option<ArcRwLockUdpSocket> = self.try_get_socket().await;
        let addr_opt: Option<SocketAddr> = self.try_get_client_addr().await;
        let response: Response = Response::from(data);
        response.send(&socket_opt, &addr_opt).await
    }
}
