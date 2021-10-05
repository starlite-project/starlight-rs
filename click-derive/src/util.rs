use syn::Ident;

pub trait LitExt {
    fn to_str(&self) -> String;
    fn to_bool(&self) -> bool;
    fn to_ident(&self) -> Ident;
}