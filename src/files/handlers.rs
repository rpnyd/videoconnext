use axum::{extract::{State, Multipart}, Json, http::StatusCode};
use std::path::PathBuf;
use tokio::fs;
use uuid::Uuid;

use crate::{state::AppState, models::UploadResponse};

// Allowed image extensions
const IMAGE_EXTS: &[&str] = &["jpg", "jpeg", "png", "gif", "webp"];
// Max file size: 10 MB
const MAX_SIZE_BYTES: usize = 10 * 1024 * 1024;

// =====================================================================
// POST /upload
// =====================================================================
// multipart/form-data request — "file" field mein file bhejo
// Header mein Bearer token zaroori hai
//
// Response:
//   { "url": "/files/abc123.jpg", "filename": "photo.jpg", "file_type": "image" }

pub async fn upload_file(
    State(_state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>, (StatusCode, String)> {

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?
    {
        // Sirf "file" field process karo
        let field_name = field.name().unwrap_or("").to_string();
        if field_name != "file" {
            continue;
        }

        let original_name = field
            .file_name()
            .unwrap_or("upload")
            .to_string();

        // Extension nikalo
        let ext = original_name
            .rsplit('.')
            .next()
            .unwrap_or("bin")
            .to_lowercase();

        // File type decide karo
        let file_type = if IMAGE_EXTS.contains(&ext.as_str()) {
            "image"
        } else {
            "file"
        };

        // Data read karo
        let data = field
            .bytes()
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

        // Size check
        if data.len() > MAX_SIZE_BYTES {
            return Err((StatusCode::PAYLOAD_TOO_LARGE, "File 10MB se badi hai".to_string()));
        }

        // Unique naam do
        let saved_name = format!("{}.{}", Uuid::new_v4(), ext);
        let path = PathBuf::from("uploads").join(&saved_name);

        // Disk pe save karo
        fs::write(&path, &data)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        tracing::info!("File upload: {} → {}", original_name, saved_name);

        return Ok(Json(UploadResponse {
            url:       format!("/files/{}", saved_name),
            filename:  original_name,
            file_type: file_type.to_string(),
        }));
    }

    Err((StatusCode::BAD_REQUEST, "Koi file nahi mili request mein".to_string()))
}
