use std::time::Duration;

use actix::{prelude::*, Actor, AsyncContext, StreamHandler};
use actix_web_actors::ws::{self};

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
        ctx.run_interval(Duration::from_secs(5), |server, ctx| {
            info!("Sending message");
            for addr in &server.addrs {
                addr.send(Message(MSG.to_owned()))
                    .into_actor(server)
                    .then(|res, _, ctx| {
                        match res {
                            Ok(_) => (),
                            Err(e) => {
                                error!("Error sending message: {}", e);
                                ctx.stop();
                            }
                        }
                        fut::ready(())
                    })
                    .wait(ctx);

                info!("Response gotten");
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
