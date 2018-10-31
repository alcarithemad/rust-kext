#![warn(clippy::all)]
#![warn(clippy::pedantic)]
extern crate proc_macro;
extern crate proc_macro2;
use self::proc_macro::TokenStream;
extern crate quote;
extern crate syn;

use quote::quote;
use syn::fold::Fold;
use syn::Expr;

struct BytesFolder();

impl syn::fold::Fold for BytesFolder {
    fn fold_expr(&mut self, i: Expr) -> Expr {
        match i {
            Expr::Lit(syn::ExprLit {attrs: _, lit: syn::Lit::ByteStr(b)}) => {
                let mut v = b.value().clone();
                v.resize(64, 0);
                let v_final = proc_macro2::Literal::byte_string(v.as_slice());
                syn::parse_quote!(*#v_final)
            },
            any => {
                syn::fold::fold_expr(self, any)
            }
        }
    }
}

#[proc_macro]
pub fn bytes_to_u8_64(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as Expr);
    let mut folder = BytesFolder();
    let output = folder.fold_expr(input);
    TokenStream::from(quote::quote!(#output))
}
