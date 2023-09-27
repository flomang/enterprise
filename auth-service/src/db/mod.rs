use common_utils::PgPool;
use actix::prelude::{Actor, SyncContext};

pub mod model;
//pub mod repository;
mod schema;
pub struct DbExecutor(pub PgPool);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}