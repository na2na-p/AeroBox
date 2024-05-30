use actix_web::{HttpRequest, HttpResponse};

pub async fn upload_file(req: HttpRequest) -> HttpResponse {
    let key = req.match_info().get("key").unwrap();
    HttpResponse::Ok().body(format!("File uploaded with key: {}", key))
}

pub async fn get_file(req: HttpRequest) -> HttpResponse {
    let key = req.match_info().get("key").unwrap();
    HttpResponse::Ok().body(format!("PreSigned URL for key: {}", key))
}

pub async fn delete_file(req: HttpRequest) -> HttpResponse {
    let key = req.match_info().get("key").unwrap();
    HttpResponse::Ok().body(format!("File deleted with key: {}", key))
}
