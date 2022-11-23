use actix::{Actor, StreamHandler};
use actix_web_actors::ws;

pub struct FakeIrc;

impl Actor for FakeIrc {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for FakeIrc {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                info!("Received: {}", text);
                ctx.text(text)
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}
