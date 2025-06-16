use crate::infra::sqlite_post_repo::SqlitePostRepo;

#[derive(Clone)]
pub struct AppState {
    pub repo: SqlitePostRepo,
}

impl AppState {
    pub fn new(repo: SqlitePostRepo) -> Self {
        Self { repo }
    }
}
