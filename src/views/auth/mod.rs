use actix_web::web;
mod login;
mod logout;
use super::path::Path;

pub fn auth_factory(app: &mut web::ServiceConfig) {
  let base_path: Path = Path { prefix: String::from("/auth"), backend: true };

  app.route(&base_path.define(String::from("/login")), web::post().to(login::login))
     .route(&base_path.define(String::from("/logout")), web::post().to(logout::logout));
}
