use service::sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db_con: DatabaseConnection,
}
