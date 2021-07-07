use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::ItemFn;

#[proc_macro_attribute] // 2
pub fn to_string(_attr: TokenStream, mut item: TokenStream) -> TokenStream {
    let string = item.to_string();
    let item2 = item.clone();
    let parsed = parse_macro_input!(item2 as ItemFn); // 3
    item.extend(generate_to_string(parsed, string)); // 4
    item
}

fn generate_to_string(parsed: ItemFn, string: String) -> TokenStream {
    /*let fn_body = parsed.block; // function body
    let sig = parsed.sig; // function signature
    let vis = parsed.vis; // visibility, pub or not
    let fn_args = sig.inputs; // comma separated args
    let fn_return_type = sig.output; // return type
    */
    let fn_name = parsed.sig.ident; // function name/identifier
    let new_name = fn_name.to_string() + "_to_string";
    let new_name = syn::Ident::new(&new_name, proc_macro2::Span::call_site());
    let x = quote::quote! {
        fn #new_name() -> &'static str {
          #string
        }
    };
    x.into()
}
