pub use sea_orm_migration::prelude::*;

mod m20230614_160038_create_table_nodes;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20230614_160038_create_table_nodes::Migration)]
    }
}
