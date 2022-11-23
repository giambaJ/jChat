use std::time::Duration;

use actix::{prelude::*, Actor, AsyncContext, StreamHandler};
use actix_web_actors::ws::{self};

/// Chat server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

pub struct FakeIrc {
    addrs: Vec<Recipient<Message>>,
};

impl Actor for FakeIrc {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(Duration::from_secs(5), || {
            self.send_message("PogU");
        });
    }
}

impl FakeIrc {
    /// Send message to all users in the room
    fn send_message(&self, message: &str) {
        for addr in self.addrs {

            addr.do_send(Message(message.to_owned()));
        }
        }
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
