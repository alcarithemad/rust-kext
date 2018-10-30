extern crate proc_macro;
extern crate proc_macro2;
use self::proc_macro::TokenStream;
extern crate quote;
extern crate syn;

use quote::quote;
use syn::parse_macro_input;
use syn::LitByteStr;
use syn::fold::Fold;

struct BytesFolder();

impl syn::fold::Fold for BytesFolder {
    fn fold_lit_byte_str(&mut self, i: LitByteStr) -> LitByteStr {
        let mut b = i.value().clone();
        b.resize(64, 0);
        LitByteStr::new(b.as_slice(), i.span())
    }
}

#[proc_macro]
pub fn bytes_to_u8_64(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::Expr);
    let mut folder = BytesFolder();
    let output = folder.fold_expr(input);
    TokenStream::from(quote!(*#output))
}
