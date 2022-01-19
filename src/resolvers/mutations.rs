use juniper::{graphql_object, FieldResult};

use crate::context::Context;
use crate::data_sources::UserSaveResult;
use crate::resolvers::errors::data_save_error;
use crate::resolvers::objects::*;

pub struct Mutation;
#[graphql_object(context = Context)]
impl Mutation {
    async fn add_user(context: &Context, input: UserInput) -> FieldResult<UserSaveMutationResult> {
        let res = context.datasources.user.create(input).await;
        fmt_user_save_result(res)
    }

    async fn update_user(
        context: &Context,
        id: i32,
        input: UserInput,
    ) -> FieldResult<UserSaveMutationResult> {
        let res = context.datasources.user.update(id, input).await;
        fmt_user_save_result(res)
    }

    async fn delete_user(context: &Context, id: i32) -> FieldResult<UserDeleteMutationResult> {
        let res = context.datasources.user.delete(id).await;
        match res {
            Ok(res) => match res {
                Ok(res) => Ok(UserDeleteMutationResult::Ok(DeletedUser { id: res })),
                Err(e) => Ok(UserDeleteMutationResult::Err(to_graphql_validation_errors(
                    e,
                ))),
            },
            Err(e) => Err(data_save_error(e)),
        }
    }

    async fn add_post(context: &Context, input: PostInput) -> Option<Post> {
        context.datasources.post.create(input).await
    }

    async fn update_post(context: &Context, id: i32, input: PostInput) -> Post {
        context.datasources.post.update(id, input).await
    }

    async fn delete_post(context: &Context, id: i32) -> i32 {
        context.datasources.post.delete(id).await
    }
}

fn fmt_user_save_result(res: UserSaveResult) -> FieldResult<UserSaveMutationResult> {
    match res {
        Ok(res) => match res {
            Ok(res) => Ok(UserSaveMutationResult::Ok(res)),
            Err(e) => Ok(UserSaveMutationResult::Err(to_graphql_validation_errors(e))),
        },
        Err(e) => Err(data_save_error(e)),
    }
}
