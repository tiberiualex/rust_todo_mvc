#[macro_use] extern crate diesel;
extern crate dotenv;

use actix_web::{ App, HttpServer, HttpResponse };
use actix_service::Service;
use futures::future::{ ok, Either };
use log;
use env_logger;

mod schema;
mod database;
mod models;
mod to_do;
mod views;
mod auth;
mod json_serialization;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        let app = App::new().wrap_fn(|req, srv| {
            let request_url: String = String::from(*&req.uri().path().clone());

            let passed: bool;

            if *&req.path().contains("/item/") {
                match auth::process_token(&req) {
                    Ok(_token) => { passed = true; },
                    Err(_message) => { passed = false; }
                }
            } else {
                passed = true;
            }

            let end_result = match passed {
                true => {
                    Either::Left(srv.call(req))
                },
                false => {
                    Either::Right(
                        ok(req.into_response(HttpResponse::Unauthorized().finish().into_body()))
                    )
                }
            };

            async move {
                let result = end_result.await?;
                log::info!("{} -> {}", request_url, &result.status());
                Ok(result)
            }
        }).configure(views::views_factory);
        return app;
    }).bind("127.0.0.1:8321")?
      .run()
      .await
}
