mod attr;
mod util;

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{Data, DeriveInput, Result, parse_macro_input};

#[proc_macro_derive(ClickCommand, attributes(buttons, styles, labels))]
pub fn derive_click(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	parse(input)
		.unwrap_or_else(|err| err.to_compile_error())
		.into()
}

fn parse(input: DeriveInput) -> Result<TokenStream2> {
	let name = input.ident;

	let value = match input.data {
		Data::Struct(data) => data,
		_ => panic!("ClickCommand can only be derived on structs"),
	};

	todo!();
}
