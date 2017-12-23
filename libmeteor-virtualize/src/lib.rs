
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
            _ => (), // TODO
        }
    }
}

impl Virtualize for BinOp {
    fn virtualize(&self, tokens: &mut Tokens) {
        use syn::BinOp::*;
        match *self {
            Add(_) => { "__add".virtualize(tokens); }
            Sub(_) => { "__sub".virtualize(tokens); },
            Mul(_) => { "__mul".virtualize(tokens); },
            Div(_) => { "__div".virtualize(tokens); },
            Rem(_) => { "__rem".virtualize(tokens); },
            And(_) => { "__and".virtualize(tokens); },
            Or(_) => { "__or".virtualize(tokens); },
            BitXor(_) => { "__bitxor".virtualize(tokens); },
            BitAnd(_) => { "__bitand".virtualize(tokens); },
            BitOr(_) => { "__bitor".virtualize(tokens); },
            Shl(_) => { "__shl".virtualize(tokens); },
            Shr(_) => { "__shr".virtualize(tokens); },
            Eq(_) => { "__eq".virtualize(tokens); },
            Lt(_) => { "__lt".virtualize(tokens); },
            Le(_) => { "__le".virtualize(tokens); },
            Ne(_) => { "__ne".virtualize(tokens); },
            Ge(_) => { "__ge".virtualize(tokens); },
            Gt(_) => { "__gt".virtualize(tokens); },
            AddEq(_) => { "__add_assign".virtualize(tokens); },
            SubEq(_) => { "__sub_assign".virtualize(tokens); },
            MulEq(_) => { "__mul_assign".virtualize(tokens); },
            DivEq(_) => { "__div_assign".virtualize(tokens); },
            RemEq(_) => { "__rem_assign".virtualize(tokens); },
            BitXorEq(_) => { "__bitxor_assign".virtualize(tokens); },
            BitAndEq(_) => { "__bitand_assign".virtualize(tokens); },
            BitOrEq(_) => { "__bitor_assign".virtualize(tokens); },
            ShlEq(_) => { "__shl_assign".virtualize(tokens); },
            ShrEq(_) => { "__shr_assign".virtualize(tokens); },
        }
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
        // TODO
    }
}

impl Virtualize for ExprBinary {
    fn virtualize(&self, tokens: &mut Tokens) {
        // {
        //     let __args = [left, right];
        //     __op(__args[0], __args[1])
        // }

        let mut lhs = Tokens::new();
        self.left.virtualize(&mut lhs);
        let mut rhs = Tokens::new();
        self.right.virtualize(&mut rhs);

        tokens::Brace::default().surround(tokens, |tokens| {
            tokens.append_tokens(
                quote! {
                    let __args = [
                        #lhs,
                        #rhs
                    ];
                }
            );

            self.op.virtualize(tokens);
            tokens::Paren::default().surround(tokens, |tokens| {
                tokens.append_tokens(
                    quote! {
                        &mut __codegen, __args[0], __args[1]
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
            let mut result = false;
            tokens.append_tokens(quote! { __Codegen::begin_scope(&mut __codegen); });
            for stmt in &self.stmts {
                match *stmt {
                    Stmt::Local(ref local) => {
                        if let Some(ref init) = local.init {
                            // panic!("{:?}", init);
                            let mut init_tokens = Tokens::new();
                            init.virtualize(&mut init_tokens);
                            tokens.append_tokens(
                                quote! {
                                    let temp = { // TODO: identifier
                                        let __args = #init_tokens;
                                        __ExprBlock::__stmnt_local(&mut __codegen, __args)
                                    };
                                }
                            );
                        } else {
                            tokens.append_tokens(
                                quote! {
                                    let temp;
                                }
                            );
                        }
                    }
                    Stmt::Item(ref item) => {
                        unimplemented!()
                    }
                    Stmt::Expr(ref expr) => {
                        let mut expr_tokens = Tokens::new();
                        expr.virtualize(&mut expr_tokens);
                        tokens.append_tokens(quote! {
                            let __result = #expr_tokens;
                        });
                        result = true;
                    }
                    Stmt::Semi(ref _expr, ref _semi) => {
                        unimplemented!()
                    }
                    Stmt::Macro(ref _mac) => {
                        unimplemented!()
                    }
                }
            }
            tokens.append_tokens(quote! { __Codegen::end_scope(&mut __codegen); });
            if result {
                tokens.append_tokens(quote! { __result });
            }
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
