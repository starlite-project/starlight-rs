use super::EntityDefinition;
use crate::make_model;
use async_trait::async_trait;
use sea_orm::{
	entity::prelude::*,
	sea_query::{ColumnDef, Table, TableCreateStatement, TableDropStatement},
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
	fn create_statement() -> TableCreateStatement {
		Table::create()
			.table(Entity)
			.if_not_exists()
			.col(ColumnDef::new(Column::Id).not_null().big_integer())
			.take()
	}

	fn drop_statement() -> TableDropStatement {
		Table::drop().table(Entity).take()
	}
}
