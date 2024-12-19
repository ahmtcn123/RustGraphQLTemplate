use axum::Router;

mod gql;

pub fn root() -> Router {
    Router::new().nest("/gql", gql::root())
}
