use actix_web::web;
mod items;
mod content_loader;
use super::path::Path;

pub fn app_factory(app: &mut web::ServiceConfig) {
    let base_path: Path = Path { prefix: String::from("/") };
    app.route(&base_path.define(String::from("")), web::get().to(items::items));
}