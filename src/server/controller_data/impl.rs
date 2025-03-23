use crate::*;

impl InnerControllerData {
    pub fn new() -> Self {
        InnerControllerData {
            socket: None,
            request: Request::new(),
            response: Response::default(),
            log: Log::default(),
            socket_addr: None,
        }
    }
}

impl ControllerData {
    pub(crate) fn from_controller_data(controller_data: InnerControllerData) -> Self {
        Self(Arc::new(RwLock::new(controller_data)))
    }

    pub async fn get_read_lock(&self) -> RwLockReadControllerData {
        let controller_data: RwLockReadControllerData = self.0.read().await;
        controller_data
    }

    pub async fn get_write_lock(&self) -> RwLockWriteControllerData {
        let controller_data: RwLockWriteControllerData = self.0.write().await;
        controller_data
    }

    pub async fn get(&self) -> InnerControllerData {
        let controller_data: InnerControllerData = self.get_read_lock().await.clone();
        controller_data
    }

    pub async fn get_request(&self) -> Request {
        let controller_data: InnerControllerData = self.get().await;
        controller_data.get_request().clone()
    }

    pub async fn get_response(&self) -> Response {
        let controller_data: InnerControllerData = self.get().await;
        controller_data.get_response().clone()
    }

    pub async fn get_log(&self) -> Log {
        let controller_data: InnerControllerData = self.get().await;
        controller_data.get_log().clone()
    }

    pub async fn get_socket(&self) -> OptionArcRwLockUdpSocket {
        let controller_data: InnerControllerData = self.get().await;
        controller_data.get_socket().clone()
    }

    pub async fn get_socket_addr(&self) -> OptionSocketAddr {
        let controller_data: InnerControllerData = self.get().await;
        let socket_addr_opt: OptionSocketAddr = controller_data.get_socket_addr().clone();
        socket_addr_opt
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
        let socket_addr_string_opt: Option<String> =
            self.get_socket_addr().await.map(|data| data.to_string());
        socket_addr_string_opt
    }

    pub async fn get_socket_addr_or_default_string(&self) -> String {
        self.get_socket_addr_or_default().await.to_string()
    }

    pub async fn get_socket_host(&self) -> OptionSocketHost {
        let addr: OptionSocketAddr = self.get_socket_addr().await;
        let socket_host_opt: OptionSocketHost =
            addr.map(|socket_addr: SocketAddr| socket_addr.ip());
        socket_host_opt
    }

    pub async fn get_socket_port(&self) -> OptionSocketPort {
        let addr: OptionSocketAddr = self.get_socket_addr().await;
        let socket_port_opt: OptionSocketPort =
            addr.map(|socket_addr: SocketAddr| socket_addr.port());
        socket_port_opt
    }

    pub(super) async fn set_response<T: Into<ResponseData>>(&self, data: T) -> &Self {
        let mut controller_data: RwLockWriteControllerData = self.get_write_lock().await;
        controller_data.set_response(server::response::r#type::Response::from(data));
        self
    }

    pub async fn log_info<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        let controller_data: RwLockReadControllerData = self.get_read_lock().await;
        let log: &Log = controller_data.get_log();
        log.info(data, func);
        self
    }

    pub async fn log_debug<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        let controller_data: RwLockReadControllerData = self.get_read_lock().await;
        let log: &Log = controller_data.get_log();
        log.debug(data, func);
        self
    }

    pub async fn log_error<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        let controller_data: RwLockReadControllerData = self.get_read_lock().await;
        let log: &Log = controller_data.get_log();
        log.error(data, func);
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
}
