use actix_web::Responder;
use super::utils::return_state;

pub async fn get() -> impl Responder {
    return return_state()
}
