use crate::infra::sqlite_post_repo::SqlitePostRepo;
use crate::infra::sqlite_user_repo::SqliteUserRepo;

#[derive(Clone)]
pub struct AppState {
    pub post_repo: SqlitePostRepo,
    pub user_repo: SqliteUserRepo,
}

impl AppState {
    pub fn new(post_repo: SqlitePostRepo, user_repo: SqliteUserRepo) -> Self {
        Self {
            post_repo,
            user_repo,
        }
    }
}
