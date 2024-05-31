use actix_web::{HttpRequest, HttpResponse, web};
use crate::services::s3::S3Service;
use std::path::Path;

pub async fn upload_file(
    req: HttpRequest,
    payload: web::Bytes,
    s3_service: web::Data<S3Service>,
) -> HttpResponse {
    let key = req.match_info().get("key").unwrap();
    let content = payload.to_vec();

    // 拡張子を取得
    let extension = Path::new(key)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_string();

    // 拡張子の長さが10バイトを超える場合はエラーを返す
    if extension.len() > 10 {
        return HttpResponse::BadRequest().body("File extension is too long");
    }

    // 拡張子を10バイトに固定する
    let mut extension_bytes = [0u8; 10];
    extension_bytes[..extension.len()].copy_from_slice(extension.as_bytes());

    // ファイルの末尾に拡張子を追加
    let mut content_with_extension = content.clone();
    content_with_extension.extend_from_slice(&extension_bytes);

    match s3_service.upload_file(key, content_with_extension).await {
        Ok(_) => HttpResponse::Ok().body(format!("/{}", key)),
        Err(e) => {
            eprintln!("Error uploading file: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to upload file")
        }
    }
}

pub async fn list_files(s3_service: web::Data<S3Service>) -> HttpResponse {
    match s3_service.list_objects().await {
        Ok(objects) => {
            let object_keys: Vec<String> = objects.into_iter().filter_map(|obj| obj.key).collect();
            HttpResponse::Ok().json(object_keys)
        },
        Err(e) => {
            eprintln!("Error listing objects: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to list objects")
        }
    }
}

pub async fn get_file(req: HttpRequest, s3_service: web::Data<S3Service>) -> HttpResponse {
    let key = req.match_info().get("key").unwrap();

    match s3_service.get_presigned_url(key).await {
        Ok(url) => HttpResponse::Found()
            .append_header(("Location", url))
            .finish(),
        Err(e) => {
            eprintln!("Error generating pre-signed URL: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to generate pre-signed URL")
        }
    }
}

pub async fn delete_file(req: HttpRequest) -> HttpResponse {
    let key = req.match_info().get("key").unwrap();
    HttpResponse::Ok().body(format!("File deleted with key: {}", key))
}
