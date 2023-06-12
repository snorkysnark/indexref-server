pub use sea_orm_migration::prelude::*;

mod m20230515_143718_create_table_scrapbook;
mod m20230609_151803_create_table_nodes;
mod m20230609_153719_create_table_telegram;
mod m20230609_160331_create_table_zotero;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230609_151803_create_table_nodes::Migration),
            Box::new(m20230515_143718_create_table_scrapbook::Migration),
            Box::new(m20230609_153719_create_table_telegram::Migration),
            Box::new(m20230609_160331_create_table_zotero::Migration),
        ]
    }
}
