extern crate actix;
extern crate actix_web;
extern crate bytes;
extern crate env_logger;
extern crate futures;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use actix_web::{
    http, middleware, server, App, AsyncResponder, Error, HttpMessage,
    HttpRequest, HttpResponse,
};

use futures::{Future};

#[derive(Debug, Serialize, Deserialize)]
struct NumberPayload {
    number: i32,
}

fn index(req: HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
    req.json()
        .from_err()
        .and_then(|incoming_payload: NumberPayload| {
            let result = 2 * incoming_payload.number;
            let response_payload = NumberPayload { number: result };
            Ok(HttpResponse::Ok().json(response_payload))
        })
        .responder()
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("json-example");

    server::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .resource("/double", |r| r.method(http::Method::POST).f(index))
    }).bind("127.0.0.1:8080")
        .unwrap()
        .shutdown_timeout(1)
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
