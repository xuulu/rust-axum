// encryption

use serde::Deserialize;
use crate::response::{
    ApiResponse,
    JsonResponse,
    ValidQuery
};

use sha2::{Digest, Sha256, Sha384, Sha512};

#[derive(Deserialize)]
pub struct QueryParams {
    md5: Option<String>,
    sha256: Option<String>,
    sha384: Option<String>,
    sha512: Option<String>,
}

#[axum::debug_handler]
pub async fn encryption(
    ValidQuery(params): ValidQuery<QueryParams>,
) -> ApiResponse {

    let result = match params {
        QueryParams { md5: Some(input), .. } => {
            let digest = md5::compute(input.as_bytes());
            format!("{:x}", digest)
        }
        QueryParams { sha256: Some(input), .. } => {
            let mut hasher = Sha256::new();
            hasher.update(input.as_bytes());
            let result = hasher.finalize();
            format!("{:x}", result)
        }
        QueryParams { sha384: Some(input), .. } => {
            let mut hasher = Sha384::new();
            hasher.update(input.as_bytes());
            let result = hasher.finalize();
            format!("{:x}", result)
        }
        QueryParams { sha512: Some(input), .. } => {
            let mut hasher = Sha512::new();
            hasher.update(input.as_bytes());
            let result = hasher.finalize();
            format!("{:x}", result)
        }
        _ => return Ok(JsonResponse::error(400, Some("请选择加密方式".to_string().parse()?))),
    };

    let data = serde_json::json!({
        "title": "哈希加密",
        "ciphertext": result
    });


    Ok(JsonResponse::success(data))
}


