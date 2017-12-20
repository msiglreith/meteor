extern crate meteor_virtualize;
extern crate syn;
extern crate synom;
#[macro_use]
extern crate quote;

use meteor_virtualize::Virtualize;
use quote::Tokens;

fn virtualize<T>(input: &str) -> String
where
    T: synom::Synom + Virtualize,
{
    let mut item = match syn::parse_str::<T>(&input) {
        Ok(item) => item,
        Err(err) => panic!("{:?}", err),
    };

    let mut tokens = Tokens::new();
    item.virtualize(&mut tokens);
    tokens.to_string()
}

#[test]
fn simple() {
    let output = virtualize::<syn::Block>(
        stringify!({
            let x = 0;
        })
    );

    let reference = quote!({
        __tokens.append("{");
        let temp = __ExprBlock::__stmnt_local(
            || 0,
            &mut __tokens);
        __tokens.append("}");
    });

    assert_eq!(output, reference.to_string());
}
