use juniper::{EmptyMutation, EmptySubscription, RootNode};

use crate::{context, resolvers};

pub type Schema = RootNode<
    'static,
    resolvers::Query,
    EmptyMutation<context::Context>,
    EmptySubscription<context::Context>,
>;

pub fn create() -> Schema {
    Schema::new(
        resolvers::Query,
        EmptyMutation::<context::Context>::new(),
        EmptySubscription::<context::Context>::new(),
    )
}
