use proc_macro::{TokenStream, TokenTree, token_stream, Ident, Group};

#[proc_macro]
pub fn replace(input: TokenStream) -> TokenStream {
    let mut it = input.into_iter();

    let replacement = get_ident(&mut it);
    let _comma = it.next().unwrap();
    let needle = get_ident(&mut it);
    let _comma = it.next().unwrap();

    rpl(replacement, needle, it.collect())
}

fn get_ident(it: &mut token_stream::IntoIter) -> Ident {
    match it.next() {
        Some(TokenTree::Ident(i)) => i,
        _ => panic!("oh noes!"),
    }
}

fn rpl(replacement: Ident, needle: Ident, input: TokenStream) -> TokenStream {
    let mut it = input.into_iter();
    it.map(|tt| match tt {
        TokenTree::Ident(i) if i.to_string() == needle.to_string() => {
            TokenTree::Ident(replacement.clone())
        },

        TokenTree::Group(g) => TokenTree::Group(Group::new(
            g.delimiter(),
            rpl(replacement.clone(), needle.clone(),g.stream()),
        )),

        other => other,
    }).collect()
}