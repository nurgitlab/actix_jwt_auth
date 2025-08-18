use actix_web::HttpMessage;
use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::services::auth_services::AuthService;

pub async fn auth_middleware_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token();

    log::debug!("Validating token: {}", token);
    
    match AuthService::validate_access_token(token) {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            Ok(req)
        },
        Err(e) => {
            log::error!("Token validation failed: {}", e);
            Err((e.into(), req))  // Теперь возвращаем кортеж (ошибка, запрос)
        },
    }
}