use crate::*;

/// Provides a default implementation for `ServerControlHook`.
impl Default for ServerControlHook {
    /// Creates a default `ServerControlHook` with no-op hooks.
    ///
    /// # Returns
    ///
    /// - `Self` - A new instance with default no-op hooks.
    #[inline(always)]
    fn default() -> Self {
        Self {
            wait_hook: Arc::new(|| Box::pin(async {})),
            shutdown_hook: Arc::new(|| Box::pin(async {})),
        }
    }
}

/// Implementation of methods for `ServerControlHook`.
impl ServerControlHook {
    /// Sets the wait hook.
    ///
    /// # Arguments
    ///
    /// - `hook` - The wait hook to set.
    pub(crate) fn set_wait_hook(
        &mut self,
        hook: Arc<dyn Fn() -> SendableAsyncTask<()> + Send + Sync>,
    ) {
        self.wait_hook = hook;
    }

    /// Sets the shutdown hook.
    ///
    /// # Arguments
    ///
    /// - `hook` - The shutdown hook to set.
    pub(crate) fn set_shutdown_hook(
        &mut self,
        hook: Arc<dyn Fn() -> SendableAsyncTask<()> + Send + Sync>,
    ) {
        self.shutdown_hook = hook;
    }

    /// Gets the wait hook.
    ///
    /// # Returns
    ///
    /// - `Arc<dyn Fn() -> SendableAsyncTask<()> + Send + Sync>` - The wait hook.
    pub fn get_wait_hook(&self) -> Arc<dyn Fn() -> SendableAsyncTask<()> + Send + Sync> {
        self.wait_hook.clone()
    }

    /// Gets the shutdown hook.
    ///
    /// # Returns
    ///
    /// - `Arc<dyn Fn() -> SendableAsyncTask<()> + Send + Sync>` - The shutdown hook.
    pub fn get_shutdown_hook(&self) -> Arc<dyn Fn() -> SendableAsyncTask<()> + Send + Sync> {
        self.shutdown_hook.clone()
    }

    /// Waits for the server to finish.
    pub async fn wait(&self) {
        (self.get_wait_hook())().await;
    }

    /// Shuts down the server.
    pub async fn shutdown(&self) {
        (self.get_shutdown_hook())().await;
    }
}

/// Implementation of methods for `HandlerState`.
impl HandlerState {
    /// Creates a new HandlerState instance.
    ///
    /// # Arguments
    ///
    /// - `socket` - The network socket.
    ///
    /// # Returns
    ///
    /// - `Self` - The newly created handler state.
    #[inline(always)]
    pub(crate) fn new(socket: ArcRwLockUdpSocket) -> Self {
        Self { socket }
    }

    /// Gets the socket.
    ///
    /// # Returns
    ///
    /// - `ArcRwLockUdpSocket` - The network socket.
    pub(crate) fn get_socket(&self) -> ArcRwLockUdpSocket {
        self.socket.clone()
    }
}

/// Implementation of `ServerHook` for `DefaultServerHook`.
impl ServerHook for DefaultServerHook {
    /// Creates a new instance of `DefaultServerHook`.
    ///
    /// # Arguments
    ///
    /// - `_ctx` - The request context (unused).
    ///
    /// # Returns
    ///
    /// A future that resolves to a new `DefaultServerHook` instance.
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    /// Executes the default hook's processing logic (no-op).
    ///
    /// # Arguments
    ///
    /// - `_ctx` - The request context (unused).
    async fn handle(self, _ctx: &Context) {}
}
