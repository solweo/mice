use leptos::{
    logging::log, 
    prelude::*
};
use crate::Note;

#[server]
pub async fn send_to_inbox(api_key: String, note: String) -> Result<(), ServerFnError> {
    type Db = surrealdb::Surreal<surrealdb::engine::any::Any>;
    
    let db = expect_context::<Db>();
    let required_key = std::env::var("MY_SECRET_API_KEY")?;

    if required_key == api_key {
        log!("Request was authenticated\nnote: {}", note.clone());
        if note.is_empty() {
            return Err(ServerFnError::ServerError(String::new()));
        }

        let _: Option<Note> = db.create("note").content(Note {
            url: note,
        }).await.unwrap();

        Ok(())
    } else {
        Err(ServerFnError::ServerError(String::new()))
    }
}

#[server]
pub async fn authenticate(api_key: String) -> Result<(), ServerFnError> {
    let required_key = std::env::var("MY_SECRET_API_KEY")?;

    if required_key == api_key {
        Ok(())
    } else {
        Err(ServerFnError::ServerError(String::from("invalid key")))
    }
}