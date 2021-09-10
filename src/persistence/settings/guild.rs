use constella::DataTransformer;
use structsy_derive::Persistent;
use twilight_model::id::GuildId;

pub type GuildKey = DataTransformer<GuildId>;

#[derive(Debug, Clone, Copy, PartialEq, Persistent)]
pub struct GuildSettings {
	#[index(mode = "exclusive")]
	raw_id: u64,
	pub id: GuildKey,
}

impl GuildSettings {
}
