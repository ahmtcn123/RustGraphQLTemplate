use async_graphql::Object;
use user::{UserMutation, UserQuery};

mod user;

pub struct QueryRoot {
    pub user_query: UserQuery,
}

#[Object]
impl QueryRoot {
    async fn user_query(&self) -> &UserQuery {
        &self.user_query
    }
}

pub struct MutationRoot {
    pub user_mutation: UserMutation,
}

#[Object]
impl MutationRoot {
    async fn user_mutation(&self) -> &UserMutation {
        &self.user_mutation
    }
}