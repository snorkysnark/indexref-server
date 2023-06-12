use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(Telegram::Table)
                    .col(
                        ColumnDef::new(Telegram::NodeId)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Telegram::ChatName).string())
                    .col(ColumnDef::new(Telegram::ChatType).string().not_null())
                    .col(ColumnDef::new(Telegram::ChatId).big_integer().not_null())
                    .col(ColumnDef::new(Telegram::Message).json_binary().not_null())
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .from(Telegram::Table, Telegram::NodeId)
                            .to(Node::Table, Node::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Telegram::Table).to_owned())
            .await?;

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Telegram {
    Table,
    NodeId,
    ChatName,
    ChatType,
    ChatId,
    Message,
}

#[derive(Iden)]
enum Node {
    Table,
    Id,
}
