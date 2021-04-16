use actix_web::dev::ServiceRequest;
pub mod jwt;
mod processes;

pub fn process_token(request: &ServiceRequest) -> Result<String, &'static str> {
    match processes::extract_header_token(request) {
        Ok(token) => {
            match processes::check_password(token) {
                Ok(token) => Ok(token),
                Err(message) => Err(message)
            }
        },
        Err(message) => Err(message)
  }
}
