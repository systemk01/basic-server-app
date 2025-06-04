use actix_web::{Responder, http::StatusCode};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApiResponse {
    pub status_code: u16,
    pub body: String,
    #[serde(skip)]
    response_code: StatusCode,
}

impl ApiResponse {
    pub fn new(status_code: u16, body: String) -> Self {
        let response_code = match status_code {
            200 => StatusCode::OK,
            201 => StatusCode::CREATED,
            204 => StatusCode::NO_CONTENT,
            400 => StatusCode::BAD_REQUEST,
            401 => StatusCode::UNAUTHORIZED,
            403 => StatusCode::FORBIDDEN,
            404 => StatusCode::NOT_FOUND,
            500 => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR, // Default case
        };

        ApiResponse {
            status_code,
            body,
            response_code,
        }
    }
}

impl Responder for ApiResponse {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponse::build(self.response_code).body(self.body)
    }
}
