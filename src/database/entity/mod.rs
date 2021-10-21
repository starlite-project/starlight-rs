pub mod guild;

use async_trait::async_trait;
use sea_orm::{ConnectionTrait, DbConn, DbErr, ExecResult, Iden, StatementBuilder, sea_query::Table};

#[doc(inline)]
pub use self::guild::GuildSettings;

#[async_trait]
pub trait EntityDefinition: Iden + Sized {
	async fn create_table(conn: &DbConn) -> Result<ExecResult, DbErr>;

	async fn drop_table(conn: &DbConn) -> Result<ExecResult, DbErr>;

    async fn execute<T: StatementBuilder + Sync>(conn: &DbConn, stmt: &T) -> Result<ExecResult, DbErr> {
        let builder = conn.get_database_backend();
        conn.execute(builder.build(stmt)).await
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! make_model {
	() => {
		#[derive(Debug, Clone, Copy, PartialEq, sea_orm::EnumIter, sea_orm::DeriveRelation)]
		pub enum Relation {}

		impl sea_orm::ActiveModelBehavior for ActiveModel {}
	};
	($name:ident) => {
		#[allow(dead_code)]
		pub type $name = self::Entity;

		make_model!();
	};
}
