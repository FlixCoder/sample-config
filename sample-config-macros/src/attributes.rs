//! Attribute parsing.

use quote::ToTokens;
use syn::{
	parse::{Parse, ParseStream},
	Attribute, Expr, Token,
};

/// A documentation comment, parsed from an attribute.
pub struct DocComment(pub Expr);

impl DocComment {
	/// Extract the documentation comments from a list of attributes, e.g. on a
	/// `Field`.
	pub fn from_attributes(attributes: &[Attribute]) -> syn::Result<Vec<Self>> {
		attributes
			.iter()
			.filter(|attr| attr.path.is_ident("doc"))
			.map(|attr| attr.tokens.clone())
			.map(syn::parse2::<Self>)
			.collect()
	}
}

impl Parse for DocComment {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		input.parse::<Token![=]>()?;
		Ok(Self(input.parse()?))
	}
}

impl ToTokens for DocComment {
	fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		self.0.to_tokens(tokens);
	}
}
