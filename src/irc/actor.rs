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

const MSG: & str = "@badge-info=subscriber/22;badges=broadcaster/1,subscriber/3012;client-nonce=6090b7621f1bf7bdcc46777cd522bca1;color=#29DE7A;display-name=sapphicjewl;emotes=;first-msg=0;flags=;id=aedfa462-66b6-4a2b-b94d-afb01d0631f9;mod=0;returning-chatter=0;room-id=538134305;subscriber=1;tmi-sent-ts=1668563455712;turbo=0;user-id=538134305;user-type= :sapphicjewl!sapphicjewl@sapphicjewl.tmi.twitch.tv PRIVMSG #sapphicjewl :monkaS\r\n";

impl Actor for FakeIrc {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let (tx, rx) = crossbeam::channel::unbounded::<String>();

        tokio::spawn(async move {
            use std::io;

            loop {
                let mut buf = String::new();

                if io::stdin().read_line(&mut buf).is_err() {
                    continue;
                }

                tx.send("wassup".to_string()).unwrap();
            }
        });

        ctx.run_interval(Duration::from_secs(1), move |_, ctx| {
            debug!("Sending message");

            let mut rng = rand::thread_rng();

            let millis: u64 = rng.gen_range(50..1500);

            thread::sleep(Duration::from_millis(millis));

            let msg = {
                rx.try_recv().unwrap_or_else(|_| {
                    crate::MESSAGES
                        .lock()
                        .pop()
                        .unwrap_or_else(|| MSG.to_string())
                })
            };

            info!("{:?}", msg);

            let parsed = crate::USERS.lock().send_message(msg);
            ctx.text(parsed);

            debug!("Response gotten");
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
