#![feature(proc_macro)]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate synom;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::Term;
use syn::*;
use syn::FilterAttrs;
use quote::Tokens;
use quote::ToTokens;

#[proc_macro_attribute]
pub fn staged(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let mut item = match syn::parse_str::<Item>(&input) {
        Ok(item) => item,
        Err(err) => panic!("{:?}", err),
    };

    // println!("{:#?}", item);

    let (mut item, block) = match item {
        Item::Fn(fn_item) => {
            if let Unsafety::Unsafe(_) = fn_item.unsafety {
                panic!("Function can't be unsafe.");
            }
            if let Constness::Const(_) = fn_item.constness {
                panic!("Function can't be const.");
            }
            if fn_item.abi.is_some() {
                panic!("Invalid function abi.");
            }
            if !fn_item.decl.generics.params.is_empty() {
                panic!("Generic types are not supported.");
            }

            let block = virtualize(&fn_item.block);
            (fn_item, block)
        }
        _ => panic!("Unexpected item kind, expected function: {:?}", item),
    };

    item.vis = syn::Visibility::Inherited(syn::VisInherited { });

    let mut outer_attrs = item.attrs.clone();

    outer_attrs.push(syn::Attribute {
        style: syn::AttrStyle::Outer,
        pound_token: syn::tokens::Pound([Span::default()]),
        bracket_token: syn::tokens::Bracket(Span::default()),
        path: syn::Path {
           leading_colon: None,
           segments: vec![
                syn::PathSegment {
                   ident: syn::Ident::new(
                       Term::intern("proc_macro"),
                       Span::default(),
                   ),
                   parameters: syn::PathParameters::None,
               }
           ].into(),
        },
        tts: vec![],
        is_sugared_doc: false,
    });

    let ident = item.ident.clone();
    let outer_ident = item.ident.clone();
    let gen_ident = syn::Ident::new(Term::intern(&(item.ident.as_ref().to_owned() + "_gen")), Span::default());

    let func = quote!{ || #block };

    println!("{:?}", func.to_string());

    let outer_item = quote! {
        #(#outer_attrs)*
        pub fn #outer_ident (input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            let generator = #func;

            // let gen_ident = quote! { #gen_ident }.to_string();
            let gen_fn = generator();
            let input = input.to_string();
            let fn_impl = format!("
                {{
                    || {{
                        {}
                    }}
                }}
            ", gen_fn.to_string());
            // panic!("{:?}", fn_impl.to_string());
            fn_impl.to_string().parse().unwrap()
        }
    };

    println!("{}", outer_item.to_string());
    outer_item.to_string().parse().unwrap()
}

fn virtualize(block: &syn::Block) -> Tokens {
    let mut tokens = Tokens::new();
    block.virtualize(&mut tokens);

    let tokens = quote! {
        {
        let mut __tokens = ::quote::Tokens::new();
        #tokens
        __tokens
        }
    };

    println!("virtualized: {}", tokens.to_string());
    tokens
}

trait Virtualize {
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
        // self.box_token.to_tokens(tokens);
        // self.expr.to_tokens(tokens);
    }
}

impl Virtualize for syn::ExprInPlace {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
        /*
        match self.kind {
            InPlaceKind::Arrow(ref arrow) => {
                self.place.to_tokens(tokens);
                arrow.to_tokens(tokens);
                self.value.to_tokens(tokens);
            }
            InPlaceKind::In(ref _in) => {
                _in.to_tokens(tokens);
                self.place.to_tokens(tokens);
                // NOTE: The second operand must be in a block, add one if
                // it is not present.
                if let ExprKind::Block(_) = self.value.node {
                    self.value.to_tokens(tokens);
                } else {
                    tokens::Brace::default().surround(tokens, |tokens| {
                        self.value.to_tokens(tokens);
                    })
                }
            }
        }
        */
    }
}

impl Virtualize for syn::ExprArray {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        self.bracket_token.surround(tokens, |tokens| {
            self.exprs.to_tokens(tokens);
        })
        */
    }
}

impl Virtualize for syn::ExprCall {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        self.func.to_tokens(tokens);
        self.paren_token.surround(tokens, |tokens| {
            self.args.to_tokens(tokens);
        })
        */
    }
}

impl Virtualize for ExprMethodCall {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        self.expr.to_tokens(tokens);
        self.dot_token.to_tokens(tokens);
        self.method.to_tokens(tokens);
        if !self.typarams.is_empty() {
            TokensOrDefault(&self.colon2_token).to_tokens(tokens);
            TokensOrDefault(&self.lt_token).to_tokens(tokens);
            self.typarams.to_tokens(tokens);
            TokensOrDefault(&self.gt_token).to_tokens(tokens);
        }
        self.paren_token.surround(tokens, |tokens| {
            self.args.to_tokens(tokens);
        });
        */
    }
}

impl Virtualize for ExprTup {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        self.paren_token.surround(tokens, |tokens| {
            self.args.to_tokens(tokens);
            // If we only have one argument, we need a trailing comma to
            // distinguish ExprTup from ExprParen.
            if self.args.len() == 1 && !self.args.trailing_delim() {
                <Token![,]>::default().to_tokens(tokens);
            }
            // XXX: Not sure how to handle this, but we never parse it yet.
            // Is this for an expression like (0,)? Can't we use the
            // trailing delimiter on Delimited for that? (,) isn't a valid
            // expression as far as I know.
            self.lone_comma.to_tokens(tokens);
        })
        */
    }
}

impl Virtualize for ExprBinary {
    fn virtualize(&self, tokens: &mut Tokens) {
        self.op.virtualize(tokens);
        tokens::Paren::default().surround(tokens, |tokens| {
            self.left.virtualize(tokens);
            tokens::Comma::default().to_tokens(tokens);
            self.right.virtualize(tokens);
        })
    }
}

impl Virtualize for ExprUnary {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        self.op.to_tokens(tokens);
        self.expr.to_tokens(tokens);
        */
    }
}

impl Virtualize for ExprCast {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        self.expr.virtualize(tokens);
        self.as_token.to_tokens(tokens);
        self.ty.to_tokens(tokens);
        */
    }
}

impl Virtualize for ExprType {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        self.expr.to_tokens(tokens);
        self.colon_token.to_tokens(tokens);
        self.ty.to_tokens(tokens);
        */
    }
}

fn maybe_wrap_else(tokens: &mut Tokens,
                   else_token: &Option<Token![else]>,
                   if_false: &Option<Box<Expr>>)
{
    /*
    if let Some(ref if_false) = *if_false {
        TokensOrDefault(&else_token).to_tokens(tokens);

        // If we are not one of the valid expressions to exist in an else
        // clause, wrap ourselves in a block.
        match if_false.node {
            ExprKind::If(_) |
            ExprKind::IfLet(_) |
            ExprKind::Block(_) => {
                if_false.to_tokens(tokens);
            }
            _ => {
                tokens::Brace::default().surround(tokens, |tokens| {
                    if_false.to_tokens(tokens);
                });
            }
        }
    }
    */
}


impl Virtualize for ExprIf {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        self.if_token.to_tokens(tokens);
        wrap_bare_struct(tokens, &self.cond);
        self.if_true.to_tokens(tokens);
        maybe_wrap_else(tokens, &self.else_token, &self.if_false);
        */
    }
}

impl Virtualize for ExprIfLet {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        self.if_token.to_tokens(tokens);
        self.let_token.to_tokens(tokens);
        self.pat.to_tokens(tokens);
        self.eq_token.to_tokens(tokens);
        wrap_bare_struct(tokens, &self.expr);
        self.if_true.to_tokens(tokens);
        maybe_wrap_else(tokens, &self.else_token, &self.if_false);
        */
    }
}

impl Virtualize for ExprWhile {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        if self.label.is_some() {
            self.label.to_tokens(tokens);
            TokensOrDefault(&self.colon_token).to_tokens(tokens);
        }
        self.while_token.to_tokens(tokens);
        wrap_bare_struct(tokens, &self.cond);
        self.body.to_tokens(tokens);
        */
    }
}

impl Virtualize for ExprWhileLet {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        if self.label.is_some() {
            self.label.to_tokens(tokens);
            TokensOrDefault(&self.colon_token).to_tokens(tokens);
        }
        self.while_token.to_tokens(tokens);
        self.let_token.to_tokens(tokens);
        self.pat.to_tokens(tokens);
        self.eq_token.to_tokens(tokens);
        wrap_bare_struct(tokens, &self.expr);
        self.body.to_tokens(tokens);
        */
    }
}

impl Virtualize for ExprForLoop {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        if self.label.is_some() {
            self.label.to_tokens(tokens);
            TokensOrDefault(&self.colon_token).to_tokens(tokens);
        }
        self.for_token.to_tokens(tokens);
        self.pat.to_tokens(tokens);
        self.in_token.to_tokens(tokens);
        wrap_bare_struct(tokens, &self.expr);
        self.body.to_tokens(tokens);
        */
    }
}

impl Virtualize for ExprLoop {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        if self.label.is_some() {
            self.label.to_tokens(tokens);
            TokensOrDefault(&self.colon_token).to_tokens(tokens);
        }
        self.loop_token.to_tokens(tokens);
        self.body.to_tokens(tokens);
        */
    }
}

impl Virtualize for ExprMatch {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        self.match_token.to_tokens(tokens);
        wrap_bare_struct(tokens, &self.expr);
        self.brace_token.surround(tokens, |tokens| {
            for (i,  arm) in self.arms.iter().enumerate() {
                arm.to_tokens(tokens);
                // Ensure that we have a comma after a non-block arm, except
                // for the last one.
                let is_last = i == self.arms.len() - 1;
                if !is_last && arm_expr_requires_comma(&arm.body) && arm.comma.is_none() {
                    <Token![,]>::default().to_tokens(tokens);
                }
            }
        });
        */
    }
}

impl Virtualize for ExprCatch {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        self.do_token.to_tokens(tokens);
        self.catch_token.to_tokens(tokens);
        self.block.to_tokens(tokens);
        */
    }
}

impl Virtualize for ExprYield {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        self.yield_token.to_tokens(tokens);
        self.expr.to_tokens(tokens);
        */
    }
}

impl Virtualize for ExprClosure {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        self.capture.to_tokens(tokens);
        self.or1_token.to_tokens(tokens);
        for item in self.decl.inputs.iter() {
            match **item.item() {
                FnArg::Captured(ArgCaptured { ref pat, ty: Type::Infer(_), .. }) => {
                    pat.to_tokens(tokens);
                }
                _ => item.item().to_tokens(tokens),
            }
            item.delimiter().to_tokens(tokens);
        }
        self.or2_token.to_tokens(tokens);
        self.decl.output.to_tokens(tokens);
        self.body.to_tokens(tokens);
        */
    }
}

impl Virtualize for ExprBlock {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        self.unsafety.to_tokens(tokens);
        self.block.to_tokens(tokens);
        */
    }
}

impl Virtualize for ExprAssign {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        self.left.to_tokens(tokens);
        self.eq_token.to_tokens(tokens);
        self.right.to_tokens(tokens);
        */
    }
}

impl Virtualize for ExprAssignOp {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        self.left.to_tokens(tokens);
        self.op.to_tokens(tokens);
        self.right.to_tokens(tokens);
        */
    }
}

impl Virtualize for ExprField {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        self.expr.to_tokens(tokens);
        self.dot_token.to_tokens(tokens);
        // XXX: I don't think we can do anything if someone shoves a
        // nonsense Lit in here.
        self.field.to_tokens(tokens);
        */
    }
}

impl Virtualize for ExprTupField {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        self.expr.to_tokens(tokens);
        self.dot_token.to_tokens(tokens);
        self.field.to_tokens(tokens);
        */
    }
}

impl Virtualize for ExprIndex {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        self.expr.to_tokens(tokens);
        self.bracket_token.surround(tokens, |tokens| {
            self.index.to_tokens(tokens);
        });
        */
    }
}

impl Virtualize for ExprRange {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        self.from.to_tokens(tokens);
        self.limits.to_tokens(tokens);
        self.to.to_tokens(tokens);
        */
    }
}

impl Virtualize for ExprPath {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        ::PathTokens(&self.qself, &self.path).to_tokens(tokens)
        */
    }
}

impl Virtualize for ExprAddrOf {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        self.and_token.to_tokens(tokens);
        self.mutbl.to_tokens(tokens);
        self.expr.to_tokens(tokens);
        */
    }
}

impl Virtualize for ExprBreak {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        self.break_token.to_tokens(tokens);
        self.label.to_tokens(tokens);
        self.expr.to_tokens(tokens);
        */
    }
}

impl Virtualize for ExprContinue {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        self.continue_token.to_tokens(tokens);
        self.label.to_tokens(tokens);
        */
    }
}

impl Virtualize for ExprRet {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        self.return_token.to_tokens(tokens);
        self.expr.virtualize(tokens);
        */
    }
}

impl Virtualize for ExprStruct {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        self.path.to_tokens(tokens);
        self.brace_token.surround(tokens, |tokens| {
            self.fields.to_tokens(tokens);
            if self.rest.is_some() {
                TokensOrDefault(&self.dot2_token).to_tokens(tokens);
                self.rest.to_tokens(tokens);
            }
        })
        */
    }
}

impl Virtualize for ExprRepeat {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        self.bracket_token.surround(tokens, |tokens| {
            self.expr.to_tokens(tokens);
            self.semi_token.to_tokens(tokens);
            self.amt.to_tokens(tokens);
        })
        */
    }
}

impl Virtualize for ExprGroup {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        self.group_token.surround(tokens, |tokens| {
            self.expr.to_tokens(tokens);
        });
        */
    }
}

impl Virtualize for ExprParen {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        self.paren_token.surround(tokens, |tokens| {
            self.expr.to_tokens(tokens);
        });
        */
    }
}

impl Virtualize for ExprTry {
    fn virtualize(&self, tokens: &mut Tokens) {
        /*
        self.expr.to_tokens(tokens);
        self.question_token.to_tokens(tokens);
        */
    }
}

impl Virtualize for syn::Block {
    fn virtualize(&self, tokens: &mut Tokens) {
        self.brace_token.surround(tokens, |tokens| {
            tokens.append_tokens(quote! { __tokens.append("{"); });
            for stmt in &self.stmts {
                match *stmt {
                    Stmt::Local(ref local) => {
                        if let Some(ref init) = local.init {
                            tokens.append_tokens(
                                quote! {
                                    let temp = __ExprBlock::__stmnt_local(|| #init, &mut __tokens);
                                }
                            );
                        } else {
                            tokens.append_tokens(
                                quote! {
                                    let temp;
                                }
                            );
                        }
                        // local.virtualize(tokens)
                    }
                    Stmt::Item(ref item) => {
                        // item.virtualize(tokens)
                        unimplemented!()
                    }
                    Stmt::Expr(ref expr) => {
                        // expr.virtualize(tokens);
                        unimplemented!()
                    }
                    Stmt::Semi(ref _expr, ref _semi) => {
                        // expr.virtualize(tokens);
                        // semi.to_tokens(tokens);
                        unimplemented!()
                    }
                    Stmt::Macro(ref _mac) => {
                        unimplemented!()
                    }
                }
                // stmt.virtualize(tokens);
            }
            tokens.append_tokens(quote! { __tokens.append("}"); });
        });
    }
}

impl Virtualize for syn::Stmt {
    fn virtualize(&self, tokens: &mut Tokens) {
        // TODO
        println!("{:#?}", self);

        /*

        match *self {
            Stmt::Local(ref local) => local.virtualize(tokens),
            Stmt::Item(ref item) => item.virtualize(tokens),
            Stmt::Expr(ref expr) => expr.virtualize(tokens),
            Stmt::Semi(ref expr, ref semi) => {
                expr.virtualize(tokens);
                semi.to_tokens(tokens);
            }
            Stmt::Macro(ref mac) => {
                let (ref mac, ref style, ref attrs) = **mac;
                tokens.append_all(attrs.outer());
                mac.to_tokens(tokens);
                match *style {
                    MacStmtStyle::Semicolon(ref s) => s.to_tokens(tokens),
                    MacStmtStyle::Braces | MacStmtStyle::NoBraces => {
                        // no semicolon
                    }
                }
            }
        }
        */
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
    /*
    if let ExprKind::Struct(_) = e.node {
        tokens::Paren::default().surround(tokens, |tokens| {
            e.to_tokens(tokens);
        });
    } else {
        e.to_tokens(tokens);
    }
    */
}
