use juniper::{EmptySubscription, RootNode};

use crate::{context, resolvers};

pub type Schema =
    RootNode<'static, resolvers::Query, resolvers::Mutation, EmptySubscription<context::Context>>;

pub fn create() -> Schema {
    Schema::new(
        resolvers::Query,
        resolvers::Mutation,
        EmptySubscription::<context::Context>::new(),
    )
}
