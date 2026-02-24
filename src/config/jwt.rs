use jsonwebtoken::{encode, Header, EncodingKey};
use std::env;
use crate::models::claims::Claims;

pub fn generar_token(claims: Claims) -> String {

    let secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET no configurado");

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref())
    ).unwrap()
}