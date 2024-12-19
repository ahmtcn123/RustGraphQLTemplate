use async_graphql::Object;
use user::{UserMutation, UserQuery};

pub mod user;

pub struct QueryRoot {
    pub user: UserQuery,
}

#[Object]
impl QueryRoot {
    async fn user(&self) -> &UserQuery {
        &self.user
    }
}

pub struct MutationRoot {
    pub user: UserMutation,
}

#[Object]
impl MutationRoot {
    async fn user(&self) -> &UserMutation {
        &self.user
    }
}

pub struct SubscriptionRoot;

#[Object]
impl SubscriptionRoot {
    async fn ping(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<String> {
        Ok("pong".to_string())
    }
}
