mod attr;

extern crate proc_macro;

use std::borrow::Borrow;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{Data, DeriveInput, Result, parse_macro_input, parse_quote};

use crate::attr::Buttons;

#[proc_macro_derive(ClickCommand, attributes(buttons, styles))]
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

	let click_attrs = attr::get(&input.attrs)?;

	todo!()
}
