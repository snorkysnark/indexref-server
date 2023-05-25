use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Zotero::Table)
                    .col(
                        ColumnDef::new(Zotero::NodeId)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Zotero::Version).integer().not_null())
                    .col(ColumnDef::new(Zotero::LibraryType).string().not_null())
                    .col(ColumnDef::new(Zotero::LibraryId).integer().not_null())
                    .col(ColumnDef::new(Zotero::LibraryName).string().not_null())
                    .col(ColumnDef::new(Zotero::LibraryLinks).string().not_null())
                    .col(ColumnDef::new(Zotero::Links).string().not_null())
                    .col(ColumnDef::new(Zotero::Meta).string().not_null())
                    .col(ColumnDef::new(Zotero::Data).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Zotero::Table, Zotero::NodeId)
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
            .drop_table(Table::drop().table(Zotero::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Zotero {
    Table,
    NodeId,
    Version,
    LibraryType,
    LibraryId,
    LibraryName,
    LibraryLinks,
    Links,
    Meta,
    Data,
}

#[derive(Iden)]
enum Node {
    Table,
    Id,
}
