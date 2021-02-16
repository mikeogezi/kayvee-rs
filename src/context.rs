use crate::store::Store;
use juniper::Context;
use std::sync::Mutex;

pub struct GraphQLContext {
  pub store: Mutex<Store>,
}

impl Context for GraphQLContext {}
