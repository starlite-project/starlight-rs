use sea_orm::entity::prelude::*;
use serde::{Serialize ,Deserialize};
use crate::make_model;

make_model!(GuildSettings);

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "guilds")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: u64
}