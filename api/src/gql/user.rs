use async_graphql::{Context, Object};
use crate::schema::user::User;

pub struct UserQuery;

#[Object]
impl UserQuery {
    
    async fn me(&self, ctx: &Context<'_>) -> async_graphql::Result<User> {
    }
}

pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn login(
        &self,
        ctx: &Context<'_>,
        username: String,
        password: String,
    ) -> async_graphql::Result<String> {
        println!("username: {username}, password: {password}");
        Ok("token".to_string())
    }
}
