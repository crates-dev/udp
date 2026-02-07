use crate::*;

/// Default host address.
pub const DEFAULT_HOST: &str = "0.0.0.0";
/// Default UDP port.
pub const DEFAULT_PORT: u16 = 60000;
/// Default buffer size for UDP packets (512KB).
pub const DEFAULT_BUFFER_SIZE: usize = 524288;
/// Default `TCP_NODELAY` setting.
pub const DEFAULT_NODELAY: Option<bool> = None;
/// Default `IP_TTL` setting.
pub const DEFAULT_TTL: Option<u32> = None;

/// Provides a default implementation for ServerConfigData.
impl Default for ServerConfigData {
    /// Creates a new ServerConfigData instance with default values.
    ///
    /// # Returns
    ///
    /// - `Self` - A new instance with default configuration.
    #[inline(always)]
    fn default() -> Self {
        Self {
            host: DEFAULT_HOST.to_owned(),
            port: DEFAULT_PORT,
            buffer_size: DEFAULT_BUFFER_SIZE,
            nodelay: DEFAULT_NODELAY,
            ttl: DEFAULT_TTL,
        }
    }
}

/// Provides a default implementation for ServerConfig.
impl Default for ServerConfig {
    /// Creates a new ServerConfig instance with default values.
    ///
    /// # Returns
    ///
    /// - `Self` - A new instance wrapping default ServerConfigData.
    #[inline(always)]
    fn default() -> Self {
        Self(arc_rwlock(ServerConfigData::default()))
    }
}

/// Implements the `PartialEq` trait for `ServerConfig`.
///
/// This allows for comparing two `ServerConfig` instances for equality.
impl PartialEq for ServerConfig {
    /// Checks if two `ServerConfig` instances are equal.
    ///
    /// It first checks for pointer equality for performance. If the pointers are not equal,
    /// it compares the inner `ServerConfigData` values.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The other `ServerConfig` to compare against.
    ///
    /// # Returns
    ///
    /// - `bool` - Indicating whether the configurations are equal.
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        if Arc::ptr_eq(&self.0, &other.0) {
            return true;
        }
        if let (Ok(s), Ok(o)) = (self.0.try_read(), other.0.try_read()) {
            *s == *o
        } else {
            false
        }
    }
}

/// Implements the `Eq` trait for `ServerConfig`.
///
/// This indicates that `ServerConfig` has a total equality relation.
impl Eq for ServerConfig {}

/// Implementation of `From` trait for `ServerConfig`.
impl From<ServerConfigData> for ServerConfig {
    /// Creates a `ServerConfig` from a `ServerConfigData`.
    ///
    /// # Arguments
    ///
    /// - `ServerConfigData` - The configuration data.
    ///
    /// # Returns
    ///
    /// - `ServerConfig` - A `ServerConfig` instance.
    #[inline(always)]
    fn from(data: ServerConfigData) -> Self {
        Self(arc_rwlock(data))
    }
}

/// Implementation block for `ServerConfig`.
impl ServerConfig {
    /// Creates a new `ServerConfig` with default values.
    ///
    /// # Returns
    ///
    /// - `Self` - A new `ServerConfig` instance.
    pub async fn new() -> Self {
        Self::default()
    }

    /// Acquires a read lock on the server configuration.
    ///
    /// # Returns
    ///
    /// - `RwLockReadGuard<ServerConfigData>` - A read guard for the inner configuration.
    async fn read(&self) -> RwLockReadGuard<'_, ServerConfigData> {
        self.0.read().await
    }

    /// Acquires a write lock on the server configuration.
    ///
    /// # Returns
    ///
    /// - `RwLockWriteGuard<ServerConfigData>` - A write guard for the inner configuration.
    async fn write(&self) -> RwLockWriteGuard<'_, ServerConfigData> {
        self.0.write().await
    }

    /// Retrieves a clone of the inner server configuration.
    ///
    /// This function provides a snapshot of the current configuration by acquiring a read lock
    /// and cloning the inner `ServerConfigData`.
    ///
    /// # Returns
    ///
    /// - `ServerConfigData` - A `ServerConfigData` instance containing the current server configuration.
    pub(crate) async fn get_data(&self) -> ServerConfigData {
        self.read().await.clone()
    }

    /// Sets the host address for the server.
    ///
    /// # Arguments
    ///
    /// - `H` - The host address to set.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to `Self` for method chaining.
    pub async fn host<H>(&self, host: H) -> &Self
    where
        H: AsRef<str>,
    {
        self.write().await.host = host.as_ref().to_owned();
        self
    }

    /// Sets the port for the server.
    ///
    /// # Arguments
    ///
    /// - `port` - The port number to set.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to `Self` for method chaining.
    pub async fn port(&self, port: u16) -> &Self {
        self.write().await.port = port;
        self
    }

    /// Sets the buffer size for receiving UDP packets.
    ///
    /// # Arguments
    ///
    /// - `size` - The buffer size in bytes.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to `Self` for method chaining.
    pub async fn buffer_size(&self, size: usize) -> &Self {
        self.write().await.buffer_size = size;
        self
    }

    /// Sets the `TCP_NODELAY` option.
    ///
    /// # Arguments
    ///
    /// - `nodelay` - The value for `TCP_NODELAY`.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to `Self` for method chaining.
    pub async fn nodelay(&self, nodelay: bool) -> &Self {
        self.write().await.nodelay = Some(nodelay);
        self
    }

    /// Enables the `TCP_NODELAY` option.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to `Self` for method chaining.
    pub async fn enable_nodelay(&self) -> &Self {
        self.nodelay(true).await
    }

    /// Disables the `TCP_NODELAY` option.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to `Self` for method chaining.
    pub async fn disable_nodelay(&self) -> &Self {
        self.nodelay(false).await
    }

    /// Sets the `IP_TTL` option.
    ///
    /// # Arguments
    ///
    /// - `ttl` - The value for `IP_TTL`.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to `Self` for method chaining.
    pub async fn ttl(&self, ttl: u32) -> &Self {
        self.write().await.ttl = Some(ttl);
        self
    }
}

/// Implementation block for `ServerConfigData`.
impl ServerConfigData {
    /// Gets the host address.
    ///
    /// # Returns
    ///
    /// - `&String` - Reference to the host address.
    pub fn get_host(&self) -> &String {
        &self.host
    }

    /// Gets the port number.
    ///
    /// # Returns
    ///
    /// - `u16` - The port number.
    pub fn get_port(&self) -> u16 {
        self.port
    }

    /// Gets the buffer size.
    ///
    /// # Returns
    ///
    /// - `usize` - The buffer size in bytes.
    pub fn get_buffer_size(&self) -> usize {
        self.buffer_size
    }

    /// Gets the `TCP_NODELAY` option.
    ///
    /// # Returns
    ///
    /// - `Option<bool>` - The `TCP_NODELAY` value if set.
    pub fn get_nodelay(&self) -> Option<bool> {
        self.nodelay
    }

    /// Gets the `IP_TTL` option.
    ///
    /// # Returns
    ///
    /// - `Option<u32>` - The `IP_TTL` value if set.
    pub fn get_ttl(&self) -> Option<u32> {
        self.ttl
    }

    /// Sets the host address.
    ///
    /// # Arguments
    ///
    /// - `impl AsRef<str>` - The host address to set.
    pub fn set_host<H>(&mut self, host: H)
    where
        H: AsRef<str>,
    {
        self.host = host.as_ref().to_owned();
    }

    /// Sets the port number.
    ///
    /// # Arguments
    ///
    /// - `u16` - The port number to set.
    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    /// Sets the buffer size.
    ///
    /// # Arguments
    ///
    /// - `usize` - The buffer size in bytes.
    pub fn set_buffer_size(&mut self, size: usize) {
        self.buffer_size = size;
    }

    /// Sets the `TCP_NODELAY` option.
    ///
    /// # Arguments
    ///
    /// - `Option<bool>` - The `TCP_NODELAY` value.
    pub fn set_nodelay(&mut self, nodelay: Option<bool>) {
        self.nodelay = nodelay;
    }

    /// Sets the `IP_TTL` option.
    ///
    /// # Arguments
    ///
    /// - `Option<u32>` - The `IP_TTL` value.
    pub fn set_ttl(&mut self, ttl: Option<u32>) {
        self.ttl = ttl;
    }
}
