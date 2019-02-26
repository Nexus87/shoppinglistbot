use rocket_contrib::json::Json;
use telegram_bot::Update;
use rocket::State;
use rocket::Route;
use services::TelegramMessageService;


#[post("/webhook", format = "json", data = "<payload>")]
fn telegram_webhook(payload: Json<Update>, telegram_service: State<Box<dyn TelegramMessageService>>) -> Result<(), ()> {
    if let Err(e) = telegram_service.handle_message(&payload){
        error!("{}", e);
    }
    Ok(())
}

pub fn get_routes() -> Vec<Route> {
    routes![telegram_webhook]
}
