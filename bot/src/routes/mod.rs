use telegram_bot::Update;
use actix::{Addr, MailboxError};
use crate::services::telegram_message_send_service::TelegramActor;
use crate::services::shopping_bot_message_service::{ShoppingBotMessageService, HandleCommand};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[post("/webhook")]
pub async fn telegram_webhook (
    payload: web::JsonBody<Update>,
    telegram_service: web::Data<Addr<ShoppingBotMessageService>>,
    _message_send_service: web::Data<Addr<TelegramActor>>,
) -> impl Responder {
    telegram_service.get_ref().send(HandleCommand{update: payload.clone()})
        .and_then(|_| {
            Ok(HttpResponse::Ok().finish())
        })
        .map_err(|_| MailboxError::Closed)
}

