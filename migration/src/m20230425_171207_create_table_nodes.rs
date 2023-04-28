use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(Node::Table)
                    .col(ColumnDef::new(Node::Id).integer().not_null().primary_key())
                    .col(ColumnDef::new(Node::Type).string().not_null())
                    .col(ColumnDef::new(Node::Title).string())
                    .col(ColumnDef::new(Node::Url).string())
                    .col(ColumnDef::new(Node::Created).string())
                    .col(ColumnDef::new(Node::File).string())
                    .col(ColumnDef::new(Node::OriginalId).string())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Node::Table).to_owned())
            .await?;

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Node {
    Table,
    Id,
    Type,
    Title,
    Url,
    Created,
    File,
    OriginalId,
}
