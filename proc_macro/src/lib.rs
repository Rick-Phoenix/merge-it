use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{ToTokens, quote, quote_spanned};
use syn::*;
use syn_utils::*;

pub(crate) struct FieldData {
	pub merge_expr: Option<PathOrClosure>,
	pub ident: Ident,
	pub type_: TokenStream2,
}

impl FieldData {
	pub fn span(&self) -> Span {
		self.ident.span()
	}
}

#[proc_macro_derive(Merge, attributes(merge))]
pub fn merge_derive(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as ItemStruct);

	handle(&input).unwrap_or_compile_error().into()
}

fn handle(input: &ItemStruct) -> syn::Result<TokenStream2> {
	let struct_ident = &input.ident;

	let mut default_expr: Option<PathOrClosure> = None;

	for attr in &input.attrs {
		if attr.path().is_ident("merge") {
			attr.parse_nested_meta(|meta| {
				let ident_str = meta.ident_str()?;

				match ident_str.as_str() {
					"with" => {
						default_expr = Some(meta.parse_value()?);
					}
					_ => return Err(meta.error("Unknown attribute")),
				};

				Ok(())
			})?;
		}
	}

	let mut fields_data: Vec<FieldData> = Vec::new();

	for field in &input.fields {
		let mut merge_expr: Option<PathOrClosure> = None;
		let mut skip = false;

		for attr in &field.attrs {
			if attr.path().is_ident("merge") {
				attr.parse_nested_meta(|meta| {
					let ident_str = meta.ident_str()?;

					match ident_str.as_str() {
						"with" => {
							merge_expr = Some(meta.parse_value()?);
						}
						"skip" => {
							skip = true;
						}
						_ => return Err(meta.error("Unknown attribute")),
					};

					Ok(())
				})?;
			}
		}

		if skip {
			continue;
		}

		fields_data.push(FieldData {
			merge_expr,
			ident: field.require_ident()?.clone(),
			type_: field.ty.to_token_stream(),
		});
	}

	let merge_fn_body = fields_data.iter().map(|data| {
		let FieldData {
			merge_expr,
			ident,
			type_,
		} = data;
		let span = data.span();

		match merge_expr.as_ref().or(default_expr.as_ref()) {
			Some(PathOrClosure::Closure(closure)) => {
				quote_spanned! {span=>
					::prelude::apply(closure, self.#ident);
				}
			}
			Some(PathOrClosure::Path(path)) => {
				quote_spanned! {span=>
					#path(&mut self.#ident, other.#ident);
				}
			}
			None => {
				quote_spanned! {span=>
					<#type_ as ::prelude::Merge>::merge(&mut self.#ident, other.#ident);
				}
			}
		}
	});

	Ok(quote! {
		impl ::prelude::Merge for #struct_ident {
			fn merge(&mut self, other: Self) {
				#(#merge_fn_body)*
			}
		}
	})
}
