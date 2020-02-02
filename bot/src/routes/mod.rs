use telegram_bot::{Update};
use crate::services::{TelegramMessageService, MessageSendService};
use std::sync::Arc;
use warp::Filter;

async fn telegram_webhook(payload: Update, telegram_service: Arc<dyn TelegramMessageService + Send>, 
                    message_send_service: Arc<dyn MessageSendService + Send>) -> Result<impl warp::Reply + Send, std::convert::Infallible> {
    let res = telegram_service.handle_message(&payload).await;
    match  res{
        Err(e) => error!("{}", e),
        Ok(Some((c, m))) => message_send_service.send_message(c, &m),
        _ => ()
    }

    Ok(warp::reply())
}

fn json_body() -> impl Filter<Extract = (Update,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    warp::body::json()
}
fn with_telegram_service(tms: Arc<dyn TelegramMessageService + Send>) -> impl Filter<Extract = (Arc<dyn TelegramMessageService + Send>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || tms.clone())
}
fn with_message_service(mss: Arc<dyn MessageSendService + Send>) -> impl Filter<Extract = (Arc<dyn MessageSendService + Send>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || mss.clone())
}
pub fn get_routes(tms: Arc<dyn TelegramMessageService + Send>, mss: Arc<dyn MessageSendService + Send>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path("webhook")
    .and(warp::post())
    .and(json_body())
    .and(with_telegram_service(tms))
    .and(with_message_service(mss))
    .and_then(telegram_webhook)
}
