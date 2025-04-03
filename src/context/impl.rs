use crate::*;

impl InnerContext {
    pub fn new() -> Self {
        InnerContext {
            socket: None,
            request: Request::new(),
            response: Response::default(),
            log: Log::default(),
            socket_addr: None,
            data: HashMap::default(),
        }
    }
}

impl Context {
    pub(crate) fn from_inner_context(ctx: InnerContext) -> Self {
        Self(Arc::new(RwLock::new(ctx)))
    }

    pub async fn get_read_lock(&self) -> RwLockReadContext {
        self.0.read().await
    }

    pub async fn get_write_lock(&self) -> RwLockWriteContext {
        self.0.write().await
    }

    pub async fn get(&self) -> InnerContext {
        self.get_read_lock().await.clone()
    }

    pub async fn get_request(&self) -> Request {
        self.get().await.get_request().clone()
    }

    pub async fn get_response(&self) -> Response {
        self.get().await.get_response().clone()
    }

    pub async fn get_log(&self) -> Log {
        self.get().await.get_log().clone()
    }

    pub async fn get_socket(&self) -> OptionArcRwLockUdpSocket {
        self.get().await.get_socket().clone()
    }

    pub async fn get_socket_addr(&self) -> OptionSocketAddr {
        self.get().await.get_socket_addr().clone()
    }

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

    pub async fn get_socket_addr_string(&self) -> Option<String> {
        self.get_socket_addr().await.map(|data| data.to_string())
    }

    pub async fn get_socket_addr_or_default_string(&self) -> String {
        self.get_socket_addr_or_default().await.to_string()
    }

    pub async fn get_socket_host(&self) -> OptionSocketHost {
        self.get_socket_addr()
            .await
            .map(|socket_addr: SocketAddr| socket_addr.ip())
    }

    pub async fn get_socket_port(&self) -> OptionSocketPort {
        self.get_socket_addr()
            .await
            .map(|socket_addr: SocketAddr| socket_addr.port())
    }

    pub(super) async fn set_response<T: Into<ResponseData>>(&self, data: T) -> &Self {
        self.get_write_lock()
            .await
            .set_response(Response::from(data));
        self
    }

    pub async fn log_info<L>(&self, data: &str, func: L) -> &Self
    where
        L: LogFuncTrait,
    {
        self.get_read_lock().await.get_log().info(data, func);
        self
    }

    pub async fn log_debug<L>(&self, data: &str, func: L) -> &Self
    where
        L: LogFuncTrait,
    {
        self.get_read_lock().await.get_log().debug(data, func);
        self
    }

    pub async fn log_error<L>(&self, data: &str, func: L) -> &Self
    where
        L: LogFuncTrait,
    {
        self.get_read_lock().await.get_log().error(data, func);
        self
    }

    pub async fn async_log_info<L>(&self, data: &str, func: L) -> &Self
    where
        L: LogFuncTrait,
    {
        self.get_read_lock()
            .await
            .get_log()
            .async_info(data, func)
            .await;
        self
    }

    pub async fn async_log_debug<L>(&self, data: &str, func: L) -> &Self
    where
        L: LogFuncTrait,
    {
        self.get_read_lock()
            .await
            .get_log()
            .async_debug(data, func)
            .await;
        self
    }

    pub async fn async_log_error<L>(&self, data: &str, func: L) -> &Self
    where
        L: LogFuncTrait,
    {
        self.get_read_lock()
            .await
            .get_log()
            .async_error(data, func)
            .await;
        self
    }

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

    pub async fn set_data_value<T: Any + Send + Sync + Clone>(
        &self,
        key: &str,
        value: &T,
    ) -> &Self {
        self.get_write_lock()
            .await
            .get_mut_data()
            .insert(key.to_owned(), Arc::new(value.clone()));
        self
    }

    pub async fn get_data_value<T: Any + Send + Sync + Clone>(&self, key: &str) -> Option<T> {
        self.get_read_lock()
            .await
            .get_data()
            .get(key)
            .and_then(|arc| arc.downcast_ref::<T>())
            .cloned()
    }

    pub async fn remove_data_value(&self, key: &str) -> &Self {
        self.get_write_lock().await.get_mut_data().remove(key);
        self
    }

    pub async fn clear_data(&self) -> &Self {
        self.get_write_lock().await.get_mut_data().clear();
        self
    }
}
