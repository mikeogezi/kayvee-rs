use juniper::FieldResult;
use juniper::RootNode;

use juniper::GraphQLObject;

use crate::context::GraphQLContext;

#[derive(GraphQLObject)]
#[graphql(description = "The result of a mutation operation")]
struct Result {
  success: bool,
}

pub struct QueryRoot {}

#[juniper::object(context = GraphQLContext)]
impl QueryRoot {
  fn get(key: String, context: &GraphQLContext) -> FieldResult<Option<String>> {
    let val = context.store.lock().unwrap().get(&key);
    Ok(val?)
  }
}

pub struct MutationRoot {}

#[juniper::object(context = GraphQLContext)]
impl MutationRoot {
  fn put(key: String, value: String, context: &GraphQLContext) -> FieldResult<Result> {
    if context.store.lock().unwrap().put(&key, &value).is_ok() {
      Ok(Result { success: true })
    } else {
      Ok(Result { success: false })
    }
  }

  fn delete(key: String, context: &GraphQLContext) -> FieldResult<Result> {
    if context.store.lock().unwrap().delete(&key).is_ok() {
      Ok(Result { success: true })
    } else {
      Ok(Result { success: false })
    }
  }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
  Schema::new(QueryRoot {}, MutationRoot {})
}
