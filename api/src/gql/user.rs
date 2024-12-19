use async_graphql::{Context, Object};
//use schema::user::User;

#[derive(Clone, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String, // In a real app, store hashed passwords
}

#[Object]
impl User {
    async fn id(&self) -> i32 {
        self.id
    }

    async fn username(&self) -> &str {
        &self.username
    }

    async fn password(&self) -> &str {
        &self.password
    }
}

pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn me(&self, ctx: &Context<'_>) -> async_graphql::Result<User> {
        Ok(User {
            id: 1,
            username: "username".to_string(),
            password: "password".to_string(),
        })
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
