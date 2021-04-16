#[macro_use] extern crate diesel;
extern crate dotenv;

use actix_web::{ App, HttpServer };
use actix_service::Service;

mod schema;
mod database;
mod processes;
mod models;
mod state;
mod to_do;
mod views;
mod auth;
mod json_serialization;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new().wrap_fn(|req, srv| {
            if *&req.path().contains("/item/") {
                match auth::process_token(&req) {
                    Ok(_token) => println!("token is passable"),
                    Err(message) => println!("token error: {}", message)
                }
            }

            let fut = srv.call(req);
            async {
                let result = fut.await?;
                Ok(result)
            }
        }).configure(views::views_factory);
        return app;
    }).bind("127.0.0.1:8321")?
      .run()
      .await
}
