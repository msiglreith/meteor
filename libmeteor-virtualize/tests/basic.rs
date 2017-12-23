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
fn assign_lit() {
    let output = virtualize::<syn::Block>(
        stringify!({ let x = 0; })
    );

    let reference = quote!({
        __Codegen::begin_scope(&mut __codegen);
        let temp = {
            let __args = 0;
            __ExprBlock::__stmnt_local(&mut __codegen, __args)
        };
        __Codegen::end_scope(&mut __codegen);
    });

    assert_eq!(output, reference.to_string());
}

#[test]
fn assign_expr_sum() {
    let output = virtualize::<syn::Block>(
        stringify!({ let x = { 1 + 2 }; })
    );

    let reference = quote!({
        __Codegen::begin_scope(&mut __codegen);
        let temp = {
            let __args = {
                __Codegen::begin_scope(&mut __codegen);
                let __result = {
                    let __args = [1, 2];
                    __add(&mut __codegen, __args[0], __args[1])
                };
                __Codegen::end_scope(&mut __codegen);
                __result
            };
            __ExprBlock::__stmnt_local(&mut __codegen, __args)
        };
        __Codegen::end_scope(&mut __codegen);
    });

    assert_eq!(output, reference.to_string());
}
