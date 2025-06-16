use crate::{config::settings::Settings, domain::user::Role};
use actix_web::{Error, HttpMessage, HttpResponse, dev::ServiceRequest, error::InternalError, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use anyhow::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    role: String,
}

pub fn create_jwt_token(user_id: Uuid, role: &Role, secret: &str) -> Result<String> {
    let exp = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp,
        role: match role {
            Role::User => "User".to_string(),
            Role::Admin => "Admin".to_string(),
        },
    };
    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?)
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<Uuid> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;
    Ok(Uuid::parse_str(&data.claims.sub)?)
}

pub async fn jwt_validator(
    req: ServiceRequest,
    creds: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let settings = if let Some(s) = req.app_data::<web::Data<Settings>>() {
        s.get_ref()
    } else {
        let body = HttpResponse::InternalServerError()
            .json(serde_json::json!({ "error": "Invalid token JWT" }));
        let err: Error = InternalError::from_response("", body).into();
        return Err((err, req));
    };

    match verify_jwt(creds.token(), &settings.jwt_secret) {
        Ok(user_id) => {
            req.extensions_mut().insert(user_id);
            Ok(req)
        }
        Err(_) => {
            let body = HttpResponse::Unauthorized()
                .json(serde_json::json!({ "error": "Invalid or expired token" }));
            let err: Error = InternalError::from_response("", body).into();
            Err((err, req))
        }
    }
}
