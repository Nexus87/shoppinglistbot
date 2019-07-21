use telegram_bot::Update;
use actix_web::{web, HttpResponse};
use futures::Future;
use actix::{Addr, MailboxError};
use services::telegram_message_send_service::TelegramActor;
use services::shopping_bot_message_service::{ShoppingBotMessageService, HandleCommand};

pub fn telegram_webhook(
    payload: web::Form<Update>,
    telegram_service: web::Data<Addr<ShoppingBotMessageService>>,
    _message_send_service: web::Data<Addr<TelegramActor>>,
) -> impl Future<Item=HttpResponse, Error=MailboxError> {
    telegram_service.send(HandleCommand{update: payload.clone()})
        .and_then(|_| {
            Ok(HttpResponse::Ok().finish())
        })
        .map_err(|_| MailboxError::Closed)
}

