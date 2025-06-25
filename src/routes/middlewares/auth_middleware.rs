use actix_web::middleware::Next;
use actix_web::{
    Error,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
};

use crate::utils::{
    api_response::{self, ApiResponse},
    jwt::decode_jwt,
};

pub async fn check_auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let auth = req.headers().get("AUTHORIZATION");

    if auth.is_none() {
        return Err(Error::from(api_response::ApiResponse::new(
            401,
            "Unauthorized".to_string(),
        )));
    }
    let token = match auth {
        Some(value) => match value.to_str() {
            Ok(header_value) => header_value.replace("Bearer ", "").to_owned(),
            Err(_) => {
                return Err(Error::from(ApiResponse::new(
                    400,
                    "Invalid authorization header format".to_string(),
                )));
            }
        },
        None => {
            return Err(Error::from(ApiResponse::new(
                401,
                "Authorization header missing".to_string(),
            )));
        }
    };

    let _claim = match decode_jwt(token) {
        Ok(claim) => claim,
        Err(_) => {
            return Err(Error::from(ApiResponse::new(
                401,
                "Invalid or expired token".to_string(),
            )));
        }
    };

    next.call(req)
        .await
        .map_err(|err| Error::from(ApiResponse::new(500, err.to_string())))
}
