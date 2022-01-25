use juniper::{graphql_object, FieldResult};

use crate::context::Context;
use crate::resolvers::errors::*;
use crate::resolvers::objects::*;

pub struct Query;
#[graphql_object(context = Context)]
impl Query {
    fn api_version() -> &'static str {
        "1.0"
    }

    async fn user(
        context: &Context,
        #[graphql(description = "id of the user")] id: i32,
    ) -> FieldResult<Option<User>> {
        let user = context.datasources.user.get_by_id(id).await;
        match user {
            Ok(user) => Ok(to_optional_graphql_user(user)),
            Err(e) => Err(data_load_error(e)),
        }
    }

    async fn users(context: &Context) -> FieldResult<Vec<User>> {
        let users = context.datasources.user.get_all().await;
        match users {
            Ok(users) => Ok(to_graphql_users(users)),
            Err(e) => Err(data_load_error(e)),
        }
    }

    async fn posts(context: &Context) -> FieldResult<Vec<Post>> {
        let posts = context.datasources.post.get_all().await;
        match posts {
            Ok(posts) => Ok(to_graphql_posts(posts)),
            Err(e) => Err(data_load_error(e)),
        }
    }

    fn hello() -> &'static str {
        "hello world!"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_sources::{mocks, models, DataSources};
    use chrono::Utc;
    use std::sync::Arc;

    mod user {
        use super::*;

        #[actix_rt::test]
        async fn 正常にデータ取得できた場合はデータをレスポンス用の構造体へマップして返すこと() {
            let mut post = mocks::Post::default();
            let post_data1 = Arc::new(models::Post {
                id: 1,
                title: "title".to_string(),
                user_id: 1,
                updated_at: Utc::now().naive_utc(),
                created_at: Utc::now().naive_utc(),
            });
            let post_data1_clone = Arc::clone(&post_data1);
            post.expect_get_all()
                .returning(move || Ok(vec![post_data1_clone.as_ref().to_owned()]));

            let mut user = mocks::User::default();

            let ctx = Context {
                datasources: DataSources { post, user },
            };

            let res = Query::posts(&ctx).await;
            let expected: FieldResult<Vec<Post>> = Ok(vec![Post {
                data: post_data1.as_ref().to_owned(),
            }]);

            assert_eq!(res, expected);
        }
    }
}
