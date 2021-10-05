use syn::{Ident, Lit};

pub trait LitExt {
	fn to_str(&self) -> String;
	fn to_bool(&self) -> bool;
	fn to_ident(&self) -> Ident;
}

impl LitExt for Lit {
	fn to_str(&self) -> String {
		match self {
			Self::Str(s) => s.value(),
			Self::ByteStr(s) => unsafe { String::from_utf8_unchecked(s.value()) },
			Self::Char(c) => c.value().to_string(),
			Self::Byte(b) => (b.value() as char).to_string(),
			_ => panic!("values must be a (byte)string or a char"),
		}
	}

	fn to_bool(&self) -> bool {
		if let Self::Bool(b) = self {
			b.value
		} else {
			self.to_str()
				.parse()
				.unwrap_or_else(|_| panic!("expected bool from {:?}", self))
		}
	}

	fn to_ident(&self) -> Ident {
		Ident::new(&self.to_str(), self.span())
	}
}
