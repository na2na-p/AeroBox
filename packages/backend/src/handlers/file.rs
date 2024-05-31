use actix_web::{HttpRequest, HttpResponse, web};
use crate::services::s3::S3Service;
use http::header::LOCATION;
use http::Uri;

pub async fn upload_file(
    req: HttpRequest,
    payload: web::Bytes,
    s3_service: web::Data<S3Service>,
) -> HttpResponse {
    let key = req.match_info().get("key").unwrap();
    let content = payload.to_vec();

    match s3_service.upload_file(key, content).await {
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
        Ok(url) => {
            let uri: Uri = url.parse().unwrap();
            HttpResponse::Found()
                .insert_header((LOCATION, uri.to_string()))
                .finish()
        },
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
