use sea_orm_migration::{prelude::*, schema::*};
use sea_orm::{DbBackend, DbConn, DbErr, Schema};

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
pub async fn setup_schema(db: &DbConn) {

    // Setup Schema helper
    let schema = Schema::new(DbBackend::Sqlite);

    // Derive from Entity
    let stmt: TableCreateStatement = schema.create_table_from_entity(crate::initializers::axum_login::user::Entity);
    let result = db
        .execute(db.get_database_backend().build(&stmt))
        .await;
    result.expect("could not crreate users table");
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
    Username,
    AccessToken,
}
