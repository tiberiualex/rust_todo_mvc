extern crate hmac;
extern crate jwt;
extern crate sha2;

use hmac:: { Hmac, NewMac };
use jwt:: { Header, Token, VerifyWithKey };
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;
use actix_web::HttpRequest;

pub struct JwtToken {
    pub user_id: i32,
    pub body: String
}

impl JwtToken {
    pub fn encode(user_id: i32) -> String {
        let key: Hmac<Sha256> = Hmac::new_varkey(b"tobereplaced").unwrap();
        let mut claims = BTreeMap::new();
        claims.insert("user_id", user_id);
        let token_str: String = claims.sign_with_key(&key).unwrap();
        return token_str;
    }

    pub fn decode(encoded_token: String) -> Result<JwtToken, &'static str> {
        let key: Hmac<Sha256> = Hmac::new_varkey(b"tobereplaced").unwrap();
        let token_str: &str = encoded_token.as_str();

        let token: Result<Token<Header, BTreeMap<String, i32>, _>, _> =
            VerifyWithKey::verify_with_key(token_str, &key);

        match token {
            Ok(token) => {
                let _header = token.header();
                let claims = token.claims();
                return Ok(JwtToken { user_id: claims["user_id"], body: encoded_token})
            }
            Err(_) => return Err("Could not decode")
        }
    }

    pub fn decode_from_request(request: HttpRequest) -> Result<JwtToken, &'static str> {
        match request.headers().get("user-token") {
            Some(token) => JwtToken::decode(String::from(token.to_str().unwrap())),
            None => Err("there is no token")
        }
    }
}

#[cfg(test)]
mod jwt_tests {
    use super::JwtToken;
    use actix_web::test;

    #[test]
    fn encode_decode() {
        let encoded_token: String = JwtToken::encode(32);
        let decoded_token: JwtToken = JwtToken::decode(encoded_token).unwrap();
        assert_eq!(32, decoded_token.user_id);
    }

    #[test]
    fn decode_incorrect_token() {
        let encoded_token: String = String::from("test");

        match JwtToken::decode(encoded_token) {
            Err(message) => assert_eq!("Could not decode", message),
            _ => panic!("Incorrect token shoult not be encoded")
        }
    }

    #[test]
    fn decode_from_request_with_correct_token() {
        let encoded_token: String = JwtToken::encode(32);
        let request = test::TestRequest::with_header("user-token", encoded_token).to_http_request();
        let outcome = JwtToken::decode_from_request(request);

        match outcome {
            Ok(token) => assert_eq!(32, token.user_id),
            _ => panic!("Token is not returned when it should be")
        }
    }

    #[test]
    fn decode_from_request_no_token() {
        let request = test::TestRequest::with_header("test", "test").to_http_request();
        let outcome = JwtToken::decode_from_request(request);
        
        match outcome {
            Err(message) => assert_eq!("there is no token", message),
            _ => panic!("Token should not be returned when it's not present in the headers")
        }
    }

    #[test]
    fn decode_from_request_with_false_token() {
        let request = test::TestRequest::with_header("user-token", "test").to_http_request();
        let outcome = JwtToken::decode_from_request(request);

        match outcome {
            Err(message) => assert_eq!("Could not decode", message),
            _ => panic!("should be an error with a fake token")
        }
    }
}