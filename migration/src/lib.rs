pub use sea_orm_migration::prelude::*;

mod m20230515_142131_create_table_nodes;
mod m20230426_114559_create_table_telegram;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230515_142131_create_table_nodes::Migration),
            Box::new(m20230426_114559_create_table_telegram::Migration),
        ]
    }
}
