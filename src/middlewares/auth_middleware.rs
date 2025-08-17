use actix_web::HttpMessage;
use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::auth::service::AuthService;
use crate::error::AuthError;

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let token = credentials.token();
    
    match AuthService::validate_access_token(token) {
        Ok(claims) => {
            // Можно добавить claims в запрос для использования в handlers
            req.extensions_mut().insert(claims);
            Ok(req)
        },
        Err(e) => Err(e.into()),
    }
}