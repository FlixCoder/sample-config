//! Derive implementations.

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Field, Fields, Type};

use crate::attributes::DocComment;

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

impl FieldData {
	/// Get `FieldData` from a field.
	fn from_field(field: Field) -> Self {
		let doc_comments =
			DocComment::from_attributes(&field.attrs).expect("Error parsing doc comments!");
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
			if <#ty as sample_config::SampleConfig>::SAMPLE_OUTPUT_TYPE == sample_config::OutputType::Value {
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
fn derive_sample_config_on_enum(ident: Ident, data: DataEnum) -> TokenStream {
	data.variants.iter().for_each(|variant| {
		if !variant.fields.is_empty() {
			unimplemented!("Enums with fields are not yet supported!");
		}
	});

	let variant_idents = data.variants.iter().map(|variant| &variant.ident);
	let variant_strings = data.variants.iter().map(|variant| variant.ident.to_string());

	#[cfg(feature = "yaml")]
	let generate_yaml = quote! {
		fn generate_sample_yaml(&self) -> String {
			match self {
				#(
					Self::#variant_idents => #variant_strings,
				)*
			}.to_owned()
		}
	};
	#[cfg(not(feature = "yaml"))]
	let generate_yaml = quote!();

	quote! {
		impl SampleConfig for #ident {
			const SAMPLE_OUTPUT_TYPE: sample_config::OutputType = sample_config::OutputType::Value;

			#generate_yaml
		}
	}
}
