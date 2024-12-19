use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::{
    response::{self, IntoResponse},
    routing::{get, post},
    Router,
};

use crate::gql::{
    user::{UserMutation, UserQuery},
    MutationRoot, QueryRoot, SubscriptionRoot,
};

async fn graphiql() -> impl IntoResponse {
    response::Html(
        GraphiQLSource::build()
            .endpoint("/gql")
            .subscription_endpoint("/gql/ws")
            .finish(),
    )
}

pub fn root() -> Router {
    let schema = Schema::build(
        QueryRoot { user: UserQuery },
        MutationRoot { user: UserMutation },
        EmptySubscription,
    )
    .finish();

    Router::new()
        .route(
            "/",
            get(graphiql).post_service(GraphQL::new(schema.clone())),
        )
        .route_service("/ws", GraphQLSubscription::new(schema))
}
