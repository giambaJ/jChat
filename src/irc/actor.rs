use actix::dev::ToEnvelope;
use actix::{
    Actor, ActorContext, ActorFuture, ActorStream, Addr, Context, ContextFutureSpawner, Handler,
    WrapFuture, WrapStream,
};
use irc::client::data::config::Config;
use irc::client::Client as IrcClient;
use irc::error::Error as IrcError;
use irc::proto::Message;

use std::mem;

/// An event produced by the client.
#[derive(Debug)]
pub enum Event {
    /// A message was received from the connection.
    Received(Message),

    /// A client error occurred.
    Error(IrcError),
}

impl actix::Message for Event {
    type Result = ();
}

/// Sends a message across the connection.
#[derive(Debug)]
pub struct SendMessage<M>(pub M);

impl<M> actix::Message for SendMessage<M> {
    type Result = Result<(), IrcError>;
}

enum State {
    Invalid,
    Started { config: Config },
    Running { client: IrcClient },
}

/// An actor wrapping an `IrcClient`.
///
/// This actor provides a basic interface for sending and receiving messages. The client actor
/// accepts a callback address for handling [`Event`]s like received messages or errors, and
/// it can handle [`SendMessage`] for sending raw messages across the connection.
///
/// [`Event`]: enum.Event.html
/// [`SendMessage`]: struct.SendMessage.html
pub struct IrcClientActor<A: Actor> {
    state: State,
    event_callback: Addr<A>,
}

impl<A: Actor> IrcClientActor<A>
where
    A: Handler<Event>,
    A::Context: ToEnvelope<A, Event>,
{
    /// Creates a new client actor with the given configuration and event callback actor.
    /// It will behave exactly as `IrcClient` does, and will automatically call
    /// `ClientExt::identify` once connected.
    pub fn start(config: Config, event_callback: Addr<A>) -> Addr<IrcClientActor<A>> {
        IrcClientActor {
            state: State::Started { config },
            event_callback,
        }
        .start()
    }

    fn take_state(&mut self) -> State {
        mem::replace(&mut self.state, State::Invalid)
    }

    fn client(&self) -> &IrcClient {
        match &self.state {
            State::Running { client, .. } => client,
            _ => panic!("Invalid state"),
        }
    }

    fn event(&self, event: Event) {
        self.event_callback.do_send(event);
    }

    fn error(&self, error: IrcError, ctx: &mut <Self as Actor>::Context) {
        self.event(Event::Error(error));
        ctx.terminate();
    }
}

impl<A: Actor> Actor for IrcClientActor<A>
where
    A: Handler<Event>,
    A::Context: ToEnvelope<A, Event>,
{
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let rt = tokio::runtime::Handle::current();

        rt.block_on(async {
            match self.take_state() {
                State::Started { config } => match IrcClient::from_config(config).await {
                    Ok(fut) => {
                        fut.into_actor(self)
                            .map(|z, act, ctx| {
                                let client = packed.0;
                                let driver = packed.1;

                                driver
                                    .into_actor(act)
                                    .map_err(|err, act, ctx| act.error(err, ctx))
                                    .spawn(ctx);

                                client
                                    .stream()
                                    .into_actor(act)
                                    .map(|msg, act, _ctx| {
                                        act.event(Event::Received(msg));
                                    })
                                    .map_err(|err, act, ctx| act.error(err, ctx))
                                    .finish()
                                    .spawn(ctx);

                                client.identify().unwrap_or_else(|err| act.error(err, ctx));

                                act.state = State::Running { client };
                            })
                            .map_err(|err, act, ctx| act.error(err, ctx))
                            .wait(ctx);
                    }
                    Err(err) => {
                        self.error(err, ctx);
                    }
                },
                _ => panic!("Invalid state"),
            }
        });
    }
}

impl<A, M> Handler<SendMessage<M>> for IrcClientActor<A>
where
    A: Actor,
    A: Handler<Event>,
    A::Context: ToEnvelope<A, Event>,
    M: Into<Message>,
{
    type Result = Result<(), IrcError>;

    fn handle(&mut self, msg: SendMessage<M>, _ctx: &mut Self::Context) -> Self::Result {
        self.client().send(msg.0)
    }
}
