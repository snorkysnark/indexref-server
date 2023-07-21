use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(File::Table)
                    .col(
                        ColumnDef::new(File::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(File::SourceType).string().not_null())
                    .col(ColumnDef::new(File::Path).string().not_null())
                    .col(ColumnDef::new(File::Hash).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Node::Table)
                    .col(
                        ColumnDef::new(Node::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Node::FileId).integer())
                    .col(ColumnDef::new(Node::NodeType).string().not_null())
                    .col(ColumnDef::new(Node::Title).string())
                    .col(ColumnDef::new(Node::Subtype).string())
                    .col(ColumnDef::new(Node::Url).string())
                    .col(ColumnDef::new(Node::Icon).string())
                    .col(ColumnDef::new(Node::Created).date_time())
                    .col(ColumnDef::new(Node::Modified).date_time())
                    .col(ColumnDef::new(Node::OriginalId).string())
                    .col(ColumnDef::new(Node::Data).json_binary())
                    .col(ColumnDef::new(Node::ParentId).integer().default(1))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Node::Table, Node::FileId)
                            .to(File::Table, File::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Node::Table, Node::ParentId)
                            .to(Node::Table, Node::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::insert()
                    .into_table(Node::Table)
                    .columns([Node::NodeType, Node::ParentId])
                    .values_panic(["Root".into(), None::<i32>.into()])
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Node::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(File::Table).to_owned())
            .await?;

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum File {
    Table,
    Id,
    SourceType,
    Path,
    Hash,
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Node {
    Table,
    Id,
    FileId,
    NodeType,
    Subtype,
    Title,
    Url,
    Created,
    Modified,
    Icon,
    OriginalId,
    Data,
    ParentId,
}
