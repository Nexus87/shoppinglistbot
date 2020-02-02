use rocket_contrib::json::Json;
use telegram_bot::{Update};
use rocket::State;
use rocket::Route;
use crate::services::{TelegramMessageService, MessageSendService};


#[post("/webhook", format = "json", data = "<payload>")]
fn telegram_webhook(payload: Json<Update>, telegram_service: State<Box<dyn TelegramMessageService>>, 
                    message_send_service: State<Box<dyn MessageSendService>>) -> Result<(), ()> {
    match telegram_service.handle_message(&payload) {
        Err(e) => error!("{}", e),
        Ok(Some((c, m))) => message_send_service.send_message(c, &m),
        _ => ()
    }

    Ok(())
}

pub fn get_routes() -> Vec<Route> {
    routes![telegram_webhook]
}
