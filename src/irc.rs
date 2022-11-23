use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

mod actor;

pub async fn handle_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(actor::FakeIrc, &req, stream);
    println!("{:?}", resp);
    resp
}
