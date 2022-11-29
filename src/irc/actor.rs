use std::{thread, time::Duration};

use actix::{prelude::*, Actor, AsyncContext, StreamHandler};
use actix_web_actors::ws::{self};
use rand::Rng;

/// Chat server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

pub struct FakeIrc {
    pub addrs: Vec<Recipient<Message>>,
}

// TODO: Add Heartbeats

impl Actor for FakeIrc {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("Creating message sender interval");

        ctx.run_interval(Duration::from_secs(1), move |_, ctx| {
            debug!("Sending message");

            let mut rng = rand::thread_rng();

            let millis: u64 = rng.gen_range(50..1500);

            info!("Sleeping for {} milliseconds", millis);

            thread::sleep(Duration::from_millis(millis));

            info!("Sending message");

            let msg = crate::MESSAGES.lock().pop_front();

            if let Some(msg) = msg {
                info!("{}", msg);

                let parsed = crate::USERS.lock().send_message(msg);
                ctx.text(parsed);

                debug!("Response gotten");
            } else {
                info!("No message to print");
            }
        });
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for FakeIrc {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                info!("Received: {}", text);
            }
            _ => (),
        }
    }
}
