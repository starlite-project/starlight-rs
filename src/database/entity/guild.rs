use super::EntityDefinition;
use crate::make_model;
use async_trait::async_trait;
use sea_orm::{
	entity::prelude::*,
	sea_query::{ColumnDef, Table},
	ExecResult, StatementBuilder,
};
use serde::{Deserialize, Serialize};

make_model!(GuildSettings);

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "guilds")]
pub struct Model {
	#[sea_orm(primary_key)]
	#[serde(skip_deserializing)]
	pub id: u64,
}

#[async_trait]
impl EntityDefinition for GuildSettings {
	async fn create_table(conn: &DbConn) -> Result<ExecResult, DbErr> {
		let table = Table::create()
		.table(Entity)
		.to_owned();

		Self::execute(conn, &table).await
	}

	async fn drop_table(conn: &DbConn) -> Result<ExecResult, DbErr> {
		let table = Table::drop().table(Entity).clone();

        Self::execute(conn, &table).await
	}
}
