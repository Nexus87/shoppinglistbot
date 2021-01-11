use std::sync::Arc;

use crate::services::{MessageSendService, TelegramMessageService};
use actix_web::{Responder, web};
use telegram_bot::Update;

pub async fn telegram_webhook(
    payload: web::Json<Update>,
    telegram_service: web::Data<Arc<dyn TelegramMessageService + Send>>,
    message_send_service: web::Data<Arc<dyn MessageSendService + Send>>,
) -> impl Responder  {
    let res = telegram_service.handle_message(&payload).await;
    match res {
        Err(e) => error!("{}", e),
        Ok(Some((c, m))) => {
            println!("{}", m);
            message_send_service.send_message(c, &m).await;
        }
        _ => (),
    }

    web::HttpResponse::Ok()
        .finish()
}