extern crate meteor_virtualize;
extern crate syn;
extern crate synom;
#[macro_use]
extern crate quote;
extern crate rustfmt_nightly as rustfmt;

use meteor_virtualize::Virtualize;
use quote::Tokens;

fn formatting(input: &str) -> String {
    let string = input.to_string();
    let config = rustfmt::config::Config::default();
    let res = rustfmt::format_input::<Vec<u8>>(
        rustfmt::Input::Text(string),
        &config,
        None,
    );

    match res {
        Ok((summary, file_map, report)) => {
            file_map[0].1.to_string()
        }
        Err(err) => {
            println!("{:?}", err);
            "".into()
        }
    }
}

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

    if true {
        let wrapper_tokens = quote! {
            fn foo() {
            let mut __tokens = ::quote::Tokens::new();
            #tokens
            __tokens
            }
        };

        println!("virtualized: {}", formatting(&wrapper_tokens.to_string()));
    }

    tokens.to_string()
}

#[test]
fn assign_lit() {
    let output = virtualize::<syn::Block>(
        stringify!({ let x = 0; })
    );

    let reference = quote!({
        __Codegen::begin_scope(&mut __codegen);
        let x = {
            let __args = 0;
            __ExprBlock::__stmnt_local(&mut __codegen, Pat::Ident("x"), __args)
        };
        let __result = __ExprBlock::__expr(&mut __codegen, Expr::empty());
        __Codegen::end_scope(&mut __codegen);
        __result
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
        let x = {
            let __args = {
                __Codegen::begin_scope(&mut __codegen);
                let __result = {
                    let __args = {
                        let __args = (1, 2);
                        __Add::__add(&mut __codegen, __args.0, __args.1)
                    };
                    __ExprBlock::__expr(&mut __codegen, __args)
                };
                __Codegen::end_scope(&mut __codegen);
                __result
            };
            __ExprBlock::__stmnt_local(&mut __codegen, Pat::Ident("x"), __args)
        };
        let __result = __ExprBlock::__expr(&mut __codegen, Expr::empty());
        __Codegen::end_scope(&mut __codegen);
        __result
    });

    assert_eq!(output, reference.to_string());
}

#[test]
fn assign_expr_sum_blocks() {
    let output = virtualize::<syn::Block>(
        stringify!({ let x = { let a0 = 0; 1 } + { let a1 = 0; 2 }; })
    );

    let reference = quote!({
        __Codegen::begin_scope(&mut __codegen);
        let x = {
            let __args = {
                let __args = (
                    {
                        __Codegen::begin_scope(&mut __codegen);
                        let a0 = {
                            let __args = 0;
                            __ExprBlock::__stmnt_local(&mut __codegen, Pat::Ident("a0"), __args)
                        };
                        let __result = {
                            let __args = 1;
                            __ExprBlock::__expr(&mut __codegen, __args)
                        };
                        __Codegen::end_scope(&mut __codegen);
                        __result
                    },
                    {
                        __Codegen::begin_scope(&mut __codegen);
                        let a1 = {
                            let __args = 0;
                            __ExprBlock::__stmnt_local(&mut __codegen, Pat::Ident("a1"), __args)
                        };
                        let __result = {
                            let __args = 2;
                            __ExprBlock::__expr(&mut __codegen, __args)
                        };
                        __Codegen::end_scope(&mut __codegen);
                        __result
                    }
                );
                __Add::__add(&mut __codegen, __args.0, __args.1)
            };
            __ExprBlock::__stmnt_local(&mut __codegen, Pat::Ident("x"), __args)
        };
        let __result = __ExprBlock::__expr(&mut __codegen, Expr::empty());
        __Codegen::end_scope(&mut __codegen);
        __result
    });

    assert_eq!(output, reference.to_string());
}

#[test]
fn assign_expr_tuple() {
    let output = virtualize::<syn::Block>(
        stringify!({ let x = (1, 2); })
    );

    let reference = quote!({
         __Codegen::begin_scope(&mut __codegen);
        let x = {
            let __args = {
                let args = (1, 2,);
                __ExprTuple::__expr(&mut __codegen, args)
            };
            __ExprBlock::__stmnt_local(&mut __codegen, Pat::Ident("x"), __args)
        };
        let __result = __ExprBlock::__expr(&mut __codegen, Expr::empty());
        __Codegen::end_scope(&mut __codegen);
        __result
    });

    assert_eq!(output, reference.to_string());
}
