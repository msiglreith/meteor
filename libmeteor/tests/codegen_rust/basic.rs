
use meteor::*;
use meteor::codegen::rust::*;

#[test]
fn expr_add() {
    let reference = {
        let mut __codegen = RustGen::new();

        let out = {
             __Codegen::begin_scope(&mut __codegen);
            let x = {
                let __args = {
                    let __args = (
                        {
                            __Codegen::begin_scope(&mut __codegen);
                            let a0 = {
                                let __args = Expr::lit(0);
                               __ExprBlock::__stmnt_local(&mut __codegen, Pat::Ident("a0"), __args)
                            };
                            let __result = {
                                let __args = Expr::lit(1);
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
                                let __args = Expr::lit(2);
                                __ExprBlock::__expr(&mut __codegen, __args)
                            };
                            __Codegen::end_scope(&mut __codegen);
                            __result
                        },
                    );
                    __Add::__add(&mut __codegen, __args.0, __args.1)
                };
                __ExprBlock::__stmnt_local(&mut __codegen, Pat::Ident("x"), __args)
            };
            let __result = __ExprBlock::__expr(&mut __codegen, Expr::empty());
            __Codegen::end_scope(&mut __codegen);
            __result
        };

        out.tokens().to_string()
    };

    assert_eq!("{ let x = { let a0 = 0i32 ; 1i32 } + { 2i32 } ; }", reference);
}

#[test]
fn expr_tuple() {
    let reference = {
        let mut __codegen = RustGen::new();

        let out = {
            __Codegen::begin_scope(&mut __codegen);
            let x = {
                let __args = {
                    let args = (Expr::lit(1), Expr::lit(2));
                    __ExprTuple::__expr(&mut __codegen, args)
                };
                __ExprBlock::__stmnt_local(&mut __codegen, Pat::Ident("x"), __args)
            };
            let __result = __ExprBlock::__expr(&mut __codegen, Expr::empty());
            __Codegen::end_scope(&mut __codegen);
            __result
        };

        out.tokens().to_string()
    };

    assert_eq!("{ let x = ( 1i32 , 2i32 ) ; }", reference);
}
