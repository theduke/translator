mod types;
mod query;
mod mutation;

use juniper::{self, RootNode, FieldResult};

pub use ::commands::{Ctx};

use self::query::Query;
use self::mutation::Mutation;

impl juniper::Context for Ctx {}


pub type Schema = RootNode<'static, Query, Mutation>;

pub fn new_schema() -> Schema {
    Schema::new(Query, Mutation)
}