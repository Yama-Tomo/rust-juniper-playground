use juniper::{graphql_object, FieldResult};

use crate::context::Context;
use crate::data_sources::{PostSaveResult, UserSaveResult};
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

    async fn add_post(context: &Context, input: PostInput) -> FieldResult<PostSaveMutationResult> {
        let res = context.datasources.post.create(input).await;
        fmt_post_save_result(res)
    }

    async fn update_post(
        context: &Context,
        id: i32,
        input: PostInput,
    ) -> FieldResult<PostSaveMutationResult> {
        let res = context.datasources.post.update(id, input).await;
        fmt_post_save_result(res)
    }

    async fn delete_post(context: &Context, id: i32) -> FieldResult<PostDeleteMutationResult> {
        let res = context.datasources.post.delete(id).await;
        match res {
            Ok(res) => match res {
                Ok(res) => Ok(PostDeleteMutationResult::Ok(DeletedPost { id: res })),
                Err(e) => Ok(PostDeleteMutationResult::Err(to_graphql_validation_errors(
                    e,
                ))),
            },
            Err(e) => Err(data_save_error(e)),
        }
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

fn fmt_post_save_result(res: PostSaveResult) -> FieldResult<PostSaveMutationResult> {
    match res {
        Ok(res) => match res {
            Ok(res) => Ok(PostSaveMutationResult::Ok(res)),
            Err(e) => Ok(PostSaveMutationResult::Err(to_graphql_validation_errors(e))),
        },
        Err(e) => Err(data_save_error(e)),
    }
}
