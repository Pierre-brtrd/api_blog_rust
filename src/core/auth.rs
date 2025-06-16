use crate::config::settings::Settings;
use actix_web::{Error, HttpMessage, HttpResponse, dev::ServiceRequest, error::InternalError, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use anyhow::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // user_id
    exp: usize,  // timestamp
}

/// Génère un JWT pour `user_id`, valide 24 h
pub fn create_jwt_token(user_id: Uuid, secret: &str) -> Result<String> {
    let exp = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp,
    };
    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?)
}

/// Vérifie le token et renvoie l’UUID du user
pub fn verify_jwt(token: &str, secret: &str) -> Result<Uuid> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;
    Ok(Uuid::parse_str(&data.claims.sub)?)
}

/// Middleware “validator” : extrait Authorization Bearer,
/// vérifie le JWT et, en cas d’échec, renvoie un 401 JSON.
///
/// Inspiré de Auth0 tuto :contentReference[oaicite:0]{index=0}
pub async fn jwt_validator(
    req: ServiceRequest,
    creds: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // Récupérer la config Settings injectée plus tard dans App::data(...)
    let settings = if let Some(s) = req.app_data::<web::Data<Settings>>() {
        s.get_ref()
    } else {
        let body = HttpResponse::InternalServerError()
            .json(serde_json::json!({ "error": "Invalid token JWT" }));
        let err: Error = InternalError::from_response("", body).into();
        return Err((err, req));
    };

    // Vérifier le token
    match verify_jwt(creds.token(), &settings.jwt_secret) {
        Ok(user_id) => {
            // on stocke l’UUID dans les extensions pour les handlers
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
