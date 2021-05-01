use actix_web::{get, web, HttpResponse};
use include_dir::{include_dir, Dir};

const STATIC_FILES: Dir = include_dir!("../frontend/dist");

#[get("/{file_name}")]
async fn service(web::Path(file_name): web::Path<String>) -> HttpResponse {
    let file = match STATIC_FILES.get_file(&file_name) {
        Some(file) => file,
        None => return HttpResponse::NotFound().finish(),
    };
    let content = match file.contents_utf8() {
        Some(content) => content,
        None => return HttpResponse::NotFound().finish(),
    };

    let content_type = mime_guess::from_path(file_name)
        .first()
        .unwrap_or(mime::TEXT_PLAIN);
    let content_type = format!("{}; charset=utf-8", content_type.to_string());

    HttpResponse::Ok().content_type(content_type).body(content)
}
