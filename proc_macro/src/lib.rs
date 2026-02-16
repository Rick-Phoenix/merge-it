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

/// Implements [`Merge`](merge_it::Merge) for a struct.
///
/// By default, each field is merged using [`Merge::merge`](merge_it::Merge::merge), which is implemented for most collections and [`Option`].
///
/// Container attributes:
///
/// - `with`
///   - Type: function path
///   - Example: `#[merge(with = my_fn)]`
///   - Description:
///		  - Sets the merging strategy for the container as a whole. Can be overridden on an individual field basis. By default, this is set to [`Merge::merge`](merge_it::Merge::merge).
///
///	Field attributes:
///
/// - `with`
///   - Type: function path or closure
///   - Example: `#[merge(with = my_fn)]` or `#[merge(with = |left, right| *left = do_something(right))]`
///   - Description:
///		  - Sets the merging strategy for the individual field. Must match the signature for [`Merge::merge`](merge_it::Merge::merge).
///
///	- `skip`
///	  - Type: Ident
///	  - Example: `#[merge(skip)]`
///	  - Description:
///	    - Ignores the field during merging.
///
///
/// # Example
///
///	```
///	use merge_it::Merge;
///
/// fn merge_double(left: &mut Vec<i32>, right: Vec<i32>) {
///     left.extend(right.into_iter().map(|num| num * 2));
/// }
///
/// #[derive(Merge, Clone)]
///	struct Example {
///	    // Uses `Merge::merge` by default
///	    simple: Vec<i32>,
///	    #[merge(with = merge_double)]
///     with_fn: Vec<i32>,
///     #[merge(with = |left, right| left.push(right[0] * 5))]
///     with_closure: Vec<i32>,
///     #[merge(skip)]
///     skipped: Vec<i32>,
///	}
///
/// #[derive(Merge, Clone)]
/// // Default logic for all fields
/// #[merge(with = merge_double)]
///	struct WithDefault {
///     uses_default: Vec<i32>,
///     // Can be overridden
///     #[merge(with = |left, right| left.push(right[0] * 5))]
///     with_override: Vec<i32>,
///	}
///
/// fn main() {
///     let mut example = Example {
///         simple: vec![1],
///         with_fn: vec![1],
///         with_closure: vec![1],
///         skipped: vec![1],
///     };
///
///     example.merge(example.clone());
///
///     assert_eq!(example.simple, [1, 1]);
///     assert_eq!(example.with_fn, [1, 2]);
///     assert_eq!(example.with_closure, [1, 5]);
///     assert_eq!(example.skipped, [1]);
///
///     let mut with_default_example = WithDefault {
///         uses_default: vec![1],
///         with_override: vec![1]
///     };
///
///     with_default_example.merge(with_default_example.clone());
///
///     assert_eq!(with_default_example.uses_default, [1, 2]);
///     assert_eq!(with_default_example.with_override, [1, 5]);
/// }
///	```
#[proc_macro_derive(Merge, attributes(merge))]
pub fn merge_derive(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as ItemStruct);

	let output = match handle(&input) {
		Ok(impl_tokens) => impl_tokens,
		Err(e) => {
			let err = e.into_compile_error();
			let struct_ident = &input.ident;

			quote! {
				impl ::merge_it::Merge for #struct_ident {
					fn merge(&mut self, other: Self) {
						unimplemented!()
					}
				}

				#err
			}
		}
	};

	output.into()
}

fn handle(input: &ItemStruct) -> syn::Result<TokenStream2> {
	let struct_ident = &input.ident;

	let mut default_expr: Option<Path> = None;

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

		if let Some(merge_expr) = merge_expr {
			match merge_expr {
				PathOrClosure::Closure(closure) => {
					quote_spanned! {span=>
						::merge_it::__apply(&mut self.#ident, other.#ident, #closure);
					}
				}
				PathOrClosure::Path(path) => {
					quote_spanned! {span=>
						#path(&mut self.#ident, other.#ident);
					}
				}
			}
		} else if let Some(path) = &default_expr {
			quote_spanned! {span=>
				#path(&mut self.#ident, other.#ident);
			}
		} else {
			quote_spanned! {span=>
				<#type_ as ::merge_it::Merge>::merge(&mut self.#ident, other.#ident);
			}
		}
	});

	Ok(quote! {
		impl ::merge_it::Merge for #struct_ident {
			fn merge(&mut self, other: Self) {
				#(#merge_fn_body)*
			}
		}
	})
}
