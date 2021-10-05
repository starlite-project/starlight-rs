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

	// for attr in input.attrs {
	// 	if attr.path.is_ident("styles") {
	// 		let value = attr::parse_values(&attr)?;
	// 		let parsed = attr::parse::<Vec<String>>(value)?;
	// 		dbg!(parsed);
	// 	}
	// 	if attr.path.is_ident("buttons") {
	// 		let value = attr::parse_values(&attr)?;
	// 		let parsed = attr::parse::<usize>(value)?;
	// 		dbg!(parsed);
	// 	}
	// }

	// let buttons_value = input.attrs.iter().find(|attr| attr.path.is_ident("buttons")).unwrap_or_else(|| panic!("expected buttons attribute"));

	let buttons_value = {
		let attribute = input.attrs.iter().find(|attr| attr.path.is_ident("buttons")).unwrap_or_else(|| panic!("expected buttons attribute"));

		let values = attr::parse_values(attribute)?;
		attr::parse::<usize>(values)?
	};

	let styles = {
		let attribute = input.attrs.iter().find(|attr| attr.path.is_ident("styles")).unwrap_or_else(|| panic!("expected styles attribute"));

		let values = attr::parse_values(attribute)?;
		attr::parse::<Vec<String>>(values)?
	};

	todo!();
}
