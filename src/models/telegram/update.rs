use serde::{Deserialize, Serialize};

use crate::{
    models::AppState,
    routes::telegram::{clear_events, sync_events},
};

use self::xl::cl_stock_update;

pub mod xl;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Update {
    pub update_id: i64,
    pub message: Option<Message>,
}
impl Update {
    pub async fn process(&self, state: &AppState) -> anyhow::Result<()> {
        match self.message.to_owned() {
            Some(message) => match message.document {
                Some(document) => {
                    let file_name = document.file_name.clone();
                    match file_name {
                        Some(name) => {
                            let file_id = document.file_id;
                            let path = state.bot.get_file(&file_id).await?;
                            let uri = format!(
                                "https://api.telegram.org/file/bot{}/{}",
                                state.bot.token(),
                                path
                            );
                            if name.contains("Carpetland") {
                                cl_stock_update(&state, &uri).await?;
                            }
                            Ok(())
                        }
                        None => Ok(()),
                    }
                }
                None => match message.text {
                    Some(text) => match text.as_str() {
                        "/sync" => {
                            let msg = sync_events(state.clone()).await?;
                            state.bot.send_message_admin(&msg).await?;
                            Ok(())
                        }
                        "/clear" => {
                            clear_events(state.clone()).await?;
                            Ok(())
                        }
                        _ => {
                            let author = match message.from {
                                Some(user) => user.first_name,
                                None => String::from("Аноним"),
                            };
                            let msg =
                                format!("Уважаемый {}, я еще не понимаю такую команду...", author);
                            state.bot.send_message_repl(message.chat.id, &msg).await?;
                            Ok(())
                        }
                    },
                    None => Ok(()),
                },
            },
            None => Ok(()),
        }
        // if payload["message"]["text"] == "/sync" {
        //     let text = telegram::sync_events(state.clone())
        //         .await
        //         .map_err(|e| crate::errors::MyError::Static(e.to_string()))?;
        //     state.bot.send_message_admin(&text).await?;
        //     Ok(StatusCode::OK)
        // } else if payload["message"]["text"] == "/clear" {
        //     telegram::clear_events(state)
        //         .await
        //         .map_err(|e| MyError::Static(e.to_string()))?;
        //     Ok(StatusCode::OK)
        // } else {
        //     let mut text: String = match serde_json::to_string_pretty(&payload) {
        //         Ok(string) => string,
        //         Err(_) => "Что-то непонятное пришло".to_string(),
        //     };
        //     text.push_str("\n\n\n из телеграм");
        //     state.bot.send_message_admin(&text).await?;
        //     Ok(StatusCode::OK)
        // }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub message_id: i64,
    pub from: Option<User>,
    pub chat: Chat,
    pub text: Option<String>,
    pub document: Option<Document>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chat {
    pub id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Document {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_name: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct File {
    pub file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileResponse {
    pub result: File,
}
