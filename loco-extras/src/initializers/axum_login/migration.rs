use sea_orm_migration::{prelude::*, schema::*};
use sea_orm::DbErr;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = table_auto(Users::Table)
            .col(pk_auto(Users::Id))
            .col(string(Users::Username).unique_key())
            .col(string(Users::AccessToken))
            .to_owned();
        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
    Username,
    AccessToken,
}
