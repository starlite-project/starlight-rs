use super::{CreateTable, DropTable};
use crate::make_model;
use sea_orm::{
	entity::prelude::*,
	sea_query::{ColumnDef, Table, TableCreateStatement, TableDropStatement},
};
use serde::{Deserialize, Serialize};

make_model!(GuildSettings);

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "guilds")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	#[serde(skip_deserializing)]
	pub id: u64,
}

impl CreateTable for GuildSettings {
	fn schema() -> TableCreateStatement {
		Table::create()
			.table(Entity)
			.if_not_exists()
			.col(
				ColumnDef::new(Column::Id)
					.not_null()
					.big_integer()
					.primary_key(),
			)
			.take()
	}
}

impl DropTable for GuildSettings {
	fn schema() -> TableDropStatement {
		Table::drop()
		.table(Entity)
		.if_exists()
		.take()
	}
}