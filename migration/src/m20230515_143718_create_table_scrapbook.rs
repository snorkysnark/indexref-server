use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Scrapbook::Table)
                    .col(
                        ColumnDef::new(Scrapbook::NodeId)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Scrapbook::About).string())
                    .col(ColumnDef::new(Scrapbook::OldId).string())
                    .col(ColumnDef::new(Scrapbook::Type).string())
                    .col(ColumnDef::new(Scrapbook::Title).string())
                    .col(ColumnDef::new(Scrapbook::Chars).string())
                    .col(ColumnDef::new(Scrapbook::Comment).string())
                    .col(ColumnDef::new(Scrapbook::Icon).string())
                    .col(ColumnDef::new(Scrapbook::Source).string())
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .from(Scrapbook::Table, Scrapbook::NodeId)
                            .to(Node::Table, Node::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Scrapbook::Table).to_owned())
            .await?;

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Scrapbook {
    Table,
    NodeId,
    About,
    OldId,
    Type,
    Title,
    Chars,
    Comment,
    Icon,
    Source,
}

#[derive(Iden)]
enum Node {
    Table,
    Id,
}
