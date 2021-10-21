pub mod guild;

#[doc(inline)]
pub use self::guild::GuildSettings;

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
    }
}