use actix_web::{get, HttpResponse};
use schemars::schema_for;
use thrive_core::{command::Command, event::Event};

#[get("/command.json")]
pub fn serve_command_schema() -> HttpResponse {
    let schema = schema_for!(Command);
    HttpResponse::Ok().body(serde_json::to_string_pretty(&schema).unwrap())
}

#[get("/event.json")]
pub fn serve_event_schema() -> HttpResponse {
    let schema = schema_for!(Event);
    HttpResponse::Ok().body(serde_json::to_string_pretty(&schema).unwrap())
}
