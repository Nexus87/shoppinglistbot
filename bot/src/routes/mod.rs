use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use crate::services::{MessageSendService, TelegramMessageService};
use telegram_bot::Update;
use crate::AppState;

#[axum::debug_handler]
pub async fn telegram_webhook(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Update>,
) -> impl IntoResponse  {
        let res = state.telegram_service.handle_message(&payload).await;
    match res {
        Err(e) => error!("{:#?}", e),
        Ok(Some((c, m))) => {
            println!("{}", m);
            state.message_send_service.send_message(c, &m).await;
        }
        _ => (),
    }
    StatusCode::OK
}