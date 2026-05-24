use crate::*;

impl ServerHook for EchoHandler {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        let request: Request = ctx.get_request().await;
        let response: String = format!("Echo: {request:?}");
        let _ = ctx.send(response).await;
    }
}

impl ServerHook for PanicHandler {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, _ctx: &Context) {
        println!("A task panic occurred");
    }
}
