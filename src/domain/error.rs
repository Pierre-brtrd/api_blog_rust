use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Un titre doit contenir au moins 3 caractères")]
    InvalidMinLentgthTitle,
    #[error("Un titre doit contenir au maximum 255 caractères")]
    InvalidMaxLentgthTitle,
    #[error("Le contenu en doit pas être vide")]
    EmptyContent,
    #[error("Erreur de base de données: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Ressource non trouvée")]
    NotFound,
    #[error("L'ID de l'utilisateur est invalide")]
    InvalidUserId,
    #[error("Erreur de hachage de mot de passe")]
    PasswordHashingError(String),
    #[error("{0}")]
    Unauthorized(String),
    #[error("Internal server error")]
    InternalError,
    #[error("This email is already used")]
    DuplicateEmail,
}
