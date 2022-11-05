//! Derive implementations.

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{
	parse::{Parse, ParseStream},
	Data, DataEnum, DataStruct, DeriveInput, Expr, Field, Fields, Token, Type,
};

/// Derive `SampleConfig* for the given derive input.
pub fn derive_sample_config(input: DeriveInput) -> TokenStream {
	if !input.generics.params.is_empty() {
		panic!("Generics are not supported!");
	}

	let ident = input.ident;
	match input.data {
		Data::Struct(data) => derive_sample_config_on_struct(ident, data),
		Data::Enum(data) => derive_sample_config_on_enum(ident, data),
		Data::Union(_) => panic!("Unions are not supported!"),
	}
}

/// Information about a field that we need.
struct FieldData {
	/// Documentation comment.
	doc_comment: TokenStream,
	/// Field identifier.
	ident: Ident,
	/// Field type.
	ty: Type,
}

/// A doc comment attribute.
struct DocComment {
	/// The actual comment expression.
	comment: Expr,
}

impl Parse for DocComment {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		input.parse::<Token![=]>()?;
		Ok(Self { comment: input.parse()? })
	}
}

impl FieldData {
	/// Get `FieldData` from a field.
	fn from_field(field: Field) -> Self {
		let doc_comments = field
			.attrs
			.into_iter()
			.filter(|attr| attr.path.is_ident("doc"))
			.map(|attr| attr.tokens)
			.map(syn::parse2::<DocComment>)
			.collect::<syn::Result<Vec<_>>>()
			.expect("Error parsing doc comments!");
		let doc_comments = doc_comments.into_iter().map(|comment| comment.comment);
		let doc_comment = quote! {
			concat!(#(#doc_comments, "\n"),*)
		};

		let ident = field.ident.expect("Field must have name!");
		let ty = field.ty;

		Self { doc_comment, ident, ty }
	}

	/// Transform the field to a `TokenStream` adding the data fields to the
	/// sample config output.
	fn to_yaml_generator(&self) -> TokenStream {
		let doc = &self.doc_comment;
		let ident = &self.ident;
		let ident_string = self.ident.to_string();
		let ty = &self.ty;

		quote! {
			let doc = #doc;
			let doc = doc.trim_end().replace('\n', "\n#");
			sample.push('#');
			sample.push_str(&doc);
			sample.push('\n');
			sample.push_str(#ident_string);
			sample.push(':');
			if #ty::SAMPLE_OUTPUT_TYPE == sample_config::OutputType::Value {
				sample.push(' ');
				sample.push_str(&self.#ident.generate_sample_yaml());
			} else {
				sample.push_str("\n  ");
				let sub_sample = self.#ident.generate_sample_yaml().replace('\n', "\n  ");
				sample.push_str(sub_sample.trim());
			}
			sample.push('\n');
		}
	}
}

/// Derive `SampleConfig` for structs.
fn derive_sample_config_on_struct(ident: Ident, data: DataStruct) -> TokenStream {
	let Fields::Named(fields) = data.fields else { panic!("Only named fields are allowed!") };
	let fields = fields.named.into_iter().map(FieldData::from_field).collect::<Vec<_>>();

	let yaml_fields = fields.iter().map(FieldData::to_yaml_generator);

	#[cfg(feature = "yaml")]
	let generate_yaml = quote! {
		fn generate_sample_yaml(&self) -> String {
			let mut sample = String::new();
			#(#yaml_fields)*
			sample
		}
	};
	#[cfg(not(feature = "yaml"))]
	let generate_yaml = quote!();

	quote! {
		impl SampleConfig for #ident {
			const SAMPLE_OUTPUT_TYPE: sample_config::OutputType = sample_config::OutputType::Fields;

			#generate_yaml
		}
	}
}

/// Derive `SampleConfig` for enums.
fn derive_sample_config_on_enum(_ident: Ident, _data: DataEnum) -> TokenStream {
	unimplemented!("Not yet implemented!")
}
