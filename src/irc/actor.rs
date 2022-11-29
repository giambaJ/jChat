use std::{thread, time::Duration};

use actix::{prelude::*, Actor, AsyncContext, StreamHandler};
use actix_web_actors::ws::{self};
use rand::Rng;

use crate::command::Command;

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

            let msg = crate::MESSAGES.lock().pop_front();

            if let Some(msg) = msg {
                // Skip any comments or empty lines
                if msg.starts_with('#') || msg.is_empty() {
                    return;
                }

                let millis: u64 = rng.gen_range(50..1500);

                debug!("Sleeping for {} milliseconds", millis);

                thread::sleep(Duration::from_millis(millis));

                debug!("Sending message");

                debug!("{}", msg);

                let parsed = Command::try_from(msg).unwrap();

                match parsed {
                    Command::Write(ref message, count) => {
                        for _ in 0..count {
                            let parsed = crate::USERS.lock().send_message(message);
                            ctx.text(parsed);

                            let millis: u64 = rng.gen_range(50..1500);

                            debug!("Sleeping for {} milliseconds", millis);

                            thread::sleep(Duration::from_millis(millis));
                        }
                    }
                    Command::Pause(millis) => thread::sleep(Duration::from_millis(millis)),
                }

                debug!("Message sent");
            } else {
                debug!("No message to print");
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
