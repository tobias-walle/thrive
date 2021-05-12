use actix_web::{dev::HttpServiceFactory, get, web, HttpResponse};
use include_dir::{include_dir, Dir};

const STATIC_FILES: Dir = include_dir!("../frontend/dist");

fn serve_file(file_name: &str) -> HttpResponse {
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

#[get("/")]
fn serve_index() -> HttpResponse {
    serve_file("index.html")
}

#[get("/{file_name}")]
fn serve_other(web::Path(file_name): web::Path<String>) -> HttpResponse {
    serve_file(&file_name)
}

pub fn service() -> impl HttpServiceFactory {
    web::scope("").service(serve_index).service(serve_other)
}
