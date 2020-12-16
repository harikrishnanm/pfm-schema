
use log::{debug, info};
use actix_web::{HttpResponse, Responder, web};


use crate::models::token::Token;

pub async fn validate(token: web::Json<Token>) -> impl Responder {
    info!("Validate token");
    debug!("Token {:?}", token);
    match token.validate().await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::Unauthorized().finish() //Todo change this to send an error
    }
}