pub mod guild;

use async_trait::async_trait;
use sea_orm::{ConnectionTrait, DbConn, DbErr, ExecResult, StatementBuilder,Iden, sea_query::{TableCreateStatement, TableDropStatement}};

#[doc(inline)]
pub use self::guild::GuildSettings;

#[async_trait]
// pub trait TableDefinition: Iden + Sized {
// 	fn create_statement() -> TableCreateStatement;

// 	fn drop_statement() -> TableDropStatement;

//     async fn execute<T: StatementBuilder + Sync>(conn: &DbConn, stmt: &T) -> Result<ExecResult, DbErr> {
//         let builder = conn.get_database_backend();
//         conn.execute(builder.build(stmt)).await
//     }

// 	async fn create_table(conn: &DbConn) -> Result<ExecResult, DbErr> {
// 		Self::execute(conn, &Self::create_statement()).await
// 	}
// }

#[async_trait]
pub trait SchemaDefinition {
	async fn execute<T: StatementBuilder + Send + Sync>(conn: &DbConn, stmt: &T) -> Result<ExecResult, DbErr> {
		let builder = conn.get_database_backend();
		conn.execute(builder.build(stmt)).await
	}
}

pub trait CreateTable: SchemaDefinition {
	fn schema() -> TableCreateStatement;
}

pub trait DropTable: SchemaDefinition {
	fn schema() -> TableDropStatement;
}

impl<T> SchemaDefinition for T where T: Iden + Sized {}

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
