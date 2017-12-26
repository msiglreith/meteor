
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate synom;
extern crate syn;

use proc_macro2::Term;
use quote::Tokens;
use quote::ToTokens;
use syn::*;

pub trait Virtualize {
    fn virtualize(&self, tokens: &mut Tokens);
}

impl Virtualize for str {
    fn virtualize(&self, tokens: &mut Tokens) {
        tokens.append(proc_macro2::TokenTree {
            span: proc_macro2::Span::default(),
            kind: proc_macro2::TokenNode::Term(Term::intern(self)),
        });
    }
}

impl Virtualize for Lit {
    fn virtualize(&self, tokens: &mut Tokens) {
        self.to_tokens(tokens)
    }
}

impl Virtualize for syn::Expr {
    fn virtualize(&self, tokens: &mut Tokens) {
        tokens.append_all(self.attrs.outer());
        self.node.virtualize(tokens);
    }
}

impl Virtualize for ExprKind {
    fn virtualize(&self, tokens: &mut Tokens) {
        match *self {
            ExprKind::Binary(ref expr) => expr.virtualize(tokens),
            ExprKind::Cast(ref expr) => expr.virtualize(tokens),
            ExprKind::Lit(ref expr) => expr.virtualize(tokens),
            ExprKind::Block(ref expr) => expr.virtualize(tokens),
            ExprKind::Tup(ref expr) => expr.virtualize(tokens),
            _ => (), // TODO
        }
    }
}

impl Virtualize for BinOp {
    fn virtualize(&self, tokens: &mut Tokens) {
        use syn::BinOp::*;
        tokens.append_tokens(
            match *self {
            Add(_) => quote! { __Add::__add },
            Sub(_) => quote! { __Sub::__sub },
            Mul(_) => quote! { __Mul::__mul },
            Div(_) => quote! { __Div::__div },
            // TODO
            Rem(_) => quote! { __rem },
            And(_) => quote! { __and },
            Or(_) => quote! { __or },
            BitXor(_) => quote! { __bitxor },
            BitAnd(_) => quote! { __bitand },
            BitOr(_) => quote! { __bitor },
            Shl(_) => quote! { __shl },
            Shr(_) => quote! { __shr },
            Eq(_) => quote! { __eq },
            Lt(_) => quote! { __lt },
            Le(_) => quote! { __le },
            Ne(_) => quote! { __ne },
            Ge(_) => quote! { __ge },
            Gt(_) => quote! { __gt },
            AddEq(_) => quote! { __add_assign },
            SubEq(_) => quote! { __sub_assign },
            MulEq(_) => quote! { __mul_assign },
            DivEq(_) => quote! { __div_assign },
            RemEq(_) => quote! { __rem_assign },
            BitXorEq(_) => quote! { __bitxor_assign },
            BitAndEq(_) => quote! { __bitand_assign },
            BitOrEq(_) => quote! { __bitor_assign },
            ShlEq(_) => quote! { __shl_assign },
            ShrEq(_) => quote! { __shr_assign },
        });
    }
}

impl Virtualize for syn::ExprBox {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for syn::ExprInPlace {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for syn::ExprArray {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for syn::ExprCall {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for ExprMethodCall {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for ExprTup {
    fn virtualize(&self, tokens: &mut Tokens) {
        // {
        //     let __args = (elem0, elem1, ..);
        //     __op(__args)
        // }

        let args = self.args
            .iter()
            .map(|arg| {
                let mut tks = Tokens::new();
                arg.item().virtualize(&mut tks);
                tks
            })
            .collect::<Vec<_>>();

        tokens::Brace::default().surround(tokens, |tokens| {
            tokens.append_tokens(
                quote! {
                    let args = (
                        #(#args,)*
                    );
                    __ExprTuple::__expr(&mut __codegen, args)
                }
            );
        })
    }
}

impl Virtualize for ExprBinary {
    fn virtualize(&self, tokens: &mut Tokens) {
        // {
        //     let __args = (left, right);
        //     __op(__args.0, __args.1)
        // }

        let mut lhs = Tokens::new();
        self.left.virtualize(&mut lhs);
        let mut rhs = Tokens::new();
        self.right.virtualize(&mut rhs);

        tokens::Brace::default().surround(tokens, |tokens| {
            tokens.append_tokens(
                quote! {
                    let __args = (
                        #lhs,
                        #rhs
                    );
                }
            );
            self.op.virtualize(tokens);
            tokens::Paren::default().surround(tokens, |tokens| {
                tokens.append_tokens(
                    quote! {
                        &mut __codegen, __args.0, __args.1
                    }
                );
            })
        })
    }
}

impl Virtualize for ExprUnary {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for ExprCast {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for ExprType {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

fn maybe_wrap_else(tokens: &mut Tokens,
                   else_token: &Option<Token![else]>,
                   if_false: &Option<Box<Expr>>)
{
    // TODO
}


impl Virtualize for ExprIf {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for ExprIfLet {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for ExprWhile {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for ExprWhileLet {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for ExprForLoop {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for ExprLoop {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for ExprMatch {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for ExprCatch {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for ExprYield {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for ExprClosure {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for ExprBlock {
    fn virtualize(&self, tokens: &mut Tokens) {
        if let Unsafety::Unsafe(_) = self.unsafety {
            unimplemented!()
        }
        self.block.virtualize(tokens);
    }
}

impl Virtualize for ExprAssign {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for ExprAssignOp {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for ExprField {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for ExprTupField {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for ExprIndex {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for ExprRange {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for ExprPath {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for ExprAddrOf {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for ExprBreak {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for ExprContinue {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for ExprRet {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for ExprStruct {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for ExprRepeat {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for ExprGroup {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for ExprParen {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for ExprTry {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for syn::Block {
    fn virtualize(&self, tokens: &mut Tokens) {
        self.brace_token.surround(tokens, |tokens| {
            tokens.append_tokens(quote! { __Codegen::begin_scope(&mut __codegen); });
            let mut result = false;
            for stmt in &self.stmts {
                match *stmt {
                    Stmt::Local(ref local) => {
                        let mut pat = Tokens::new();
                        local.pat.to_tokens(&mut pat);

                        let pat_construct = match *local.pat {
                            syn::Pat::Wild(_) => quote! {
                                Pat::Wild
                            },
                            syn::Pat::Ident(ref ident) => {
                                let ident = ident.ident.to_string();
                                quote! {
                                    Pat::Ident(#ident)
                                }
                            },
                            _ => panic!("{:?}", local.pat),
                        };

                        if let Some(ref init) = local.init {
                            // panic!("{:?}", init);
                            let mut init_tokens = Tokens::new();
                            init.virtualize(&mut init_tokens);
                            tokens.append_tokens(
                                quote! {
                                    let #pat = {
                                        let __args = #init_tokens;
                                        __ExprBlock::__stmnt_local(&mut __codegen, #pat_construct, __args)
                                    };
                                }
                            );
                        } else {
                            tokens.append_tokens(
                                quote! {
                                    let #pat;
                                }
                            );
                        }
                    }
                    Stmt::Item(ref item) => {
                        unimplemented!()
                    }
                    Stmt::Expr(ref expr) => {
                        result = true;
                        let mut expr_tokens = Tokens::new();
                        expr.virtualize(&mut expr_tokens);
                        tokens.append_tokens(quote! {
                            let __result = {
                                let __args = #expr_tokens;
                                __ExprBlock::__expr(&mut __codegen, __args)
                            };
                        });
                    }
                    Stmt::Semi(ref _expr, ref _semi) => {
                        unimplemented!()
                    }
                    Stmt::Macro(ref _mac) => {
                        unimplemented!()
                    }
                }
            }

            if !result {
                tokens.append_tokens(quote! {
                    let __result = __ExprBlock::__expr(&mut __codegen, Expr::empty());
                });
            }

            tokens.append_tokens(quote! {
                __Codegen::end_scope(&mut __codegen);
                __result
            });
        });
    }
}

impl Virtualize for syn::Stmt {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
        println!("{:#?}", self);
    }
}

impl Virtualize for syn::Local {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

impl Virtualize for syn::Item {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
    }
}

fn wrap_bare_struct(tokens: &mut Tokens, e: &Expr) {
    // TODO
}
