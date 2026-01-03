use proc_macro2::{extra::DelimSpan, LineColumn, Span};
use smallvec::{smallvec, SmallVec};
use smol_str::ToSmolStr;
use syn::{
    punctuated::{self, Punctuated},
    spanned::Spanned,
    token, Abi, Attribute, Block, Expr, ExprArray, ExprForLoop, ExprLit, ExprReference, File,
    FnArg, GenericParam, Generics, Ident, Item, ItemConst, ItemFn, Label, Lifetime, Lit, LitStr,
    Pat, Path, PathArguments, PathSegment, QSelf, ReturnType, Signature, Stmt, Type, TypePath,
    TypeReference, TypeSlice, Variadic, Visibility, WhereClause,
};

use crate::{Error, Location, Node, NodeChild, Nodes, Parse, Position, Range, Value};

impl<'a> From<&'a File> for Node {
    fn from(value: &'a File) -> Self {
        Self::new(
            "File".to_smolstr(),
            Some(value.span().into()),
            vec![
                NodeChild::new("shebang".to_smolstr(), from_option_string(&value.shebang)),
                NodeChild::new("attrs".to_smolstr(), from_slice(&value.attrs)),
                NodeChild::new("items".to_smolstr(), from_slice(&value.items)),
                span_child(value),
            ],
        )
    }
}

impl<'a> From<&'a Attribute> for Node {
    fn from(value: &'a Attribute) -> Self {
        unimplemented!()
    }
}

impl<'a> From<&'a Attribute> for Value {
    fn from(value: &'a Attribute) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a Item> for Node {
    fn from(value: &'a Item) -> Self {
        match value {
            Item::Const(item) => item.into(),
            Item::Fn(item) => item.into(),
            _ => unimplemented!(),
        }
    }
}

impl<'a> From<&'a Item> for Value {
    fn from(value: &'a Item) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a ItemConst> for Node {
    fn from(value: &'a ItemConst) -> Self {
        Self::new(
            "ItemConst".to_smolstr(),
            Some(value.span().into()),
            vec![
                NodeChild::new("attrs".to_smolstr(), from_slice(&value.attrs)),
                NodeChild::new("vis".to_smolstr(), (&value.vis).into()),
                NodeChild::new("const_token".to_smolstr(), (&value.const_token).into()),
                NodeChild::new("ident".to_smolstr(), (&value.ident).into()),
                NodeChild::new("generics".to_smolstr(), (&value.generics).into()),
                NodeChild::new("colon_token".to_smolstr(), (&value.colon_token).into()),
                NodeChild::new("ty".to_smolstr(), (&*value.ty).into()),
                NodeChild::new("eq_token".to_smolstr(), (&value.eq_token).into()),
                NodeChild::new("expr".to_smolstr(), (&*value.expr).into()),
                NodeChild::new("semi_token".to_smolstr(), (&value.semi_token).into()),
                span_child(value),
            ],
        )
    }
}

impl<'a> From<&'a ItemConst> for Value {
    fn from(value: &'a ItemConst) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a ItemFn> for Node {
    fn from(value: &'a ItemFn) -> Self {
        Self::new(
            "ItemFn".to_smolstr(),
            Some(value.span().into()),
            vec![
                NodeChild::new("attrs".to_smolstr(), from_slice(&value.attrs)),
                NodeChild::new("vis".to_smolstr(), (&value.vis).into()),
                NodeChild::new("sig".to_smolstr(), (&value.sig).into()),
                NodeChild::new("block".to_smolstr(), (&*value.block).into()),
                span_child(value),
            ],
        )
    }
}

impl<'a> From<&'a ItemFn> for Value {
    fn from(value: &'a ItemFn) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a Visibility> for Value {
    fn from(value: &'a Visibility) -> Self {
        match value {
            Visibility::Public(pub_token) => Node::new(
                "Public".to_smolstr(),
                Some(value.span().into()),
                vec![
                    NodeChild::new("pub_token".to_smolstr(), pub_token.into()),
                    span_child(value),
                ],
            )
            .into(),
            Visibility::Inherited => Self::Scalar("Inherited".to_smolstr()),
            _ => unimplemented!(),
        }
    }
}

impl<'a> From<&'a token::And> for Node {
    fn from(value: &'a token::And) -> Self {
        span_only(value, "And")
    }
}

impl<'a> From<&'a token::And> for Value {
    fn from(value: &'a token::And) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a token::Async> for Node {
    fn from(value: &'a token::Async) -> Self {
        span_only(value, "Async")
    }
}

impl<'a> From<&'a token::Async> for Value {
    fn from(value: &'a token::Async) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a token::Brace> for Node {
    fn from(value: &'a token::Brace) -> Self {
        delim_span_only(&value.span, "Brace")
    }
}

impl<'a> From<&'a token::Brace> for Value {
    fn from(value: &'a token::Brace) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a token::Bracket> for Node {
    fn from(value: &'a token::Bracket) -> Self {
        delim_span_only(&value.span, "Bracket")
    }
}

impl<'a> From<&'a token::Bracket> for Value {
    fn from(value: &'a token::Bracket) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a token::Colon> for Node {
    fn from(value: &'a token::Colon) -> Self {
        span_only(value, "Colon")
    }
}

impl<'a> From<&'a token::Colon> for Value {
    fn from(value: &'a token::Colon) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a token::Comma> for Node {
    fn from(value: &'a token::Comma) -> Self {
        span_only(value, "Comma")
    }
}

impl<'a> From<&'a token::Comma> for Value {
    fn from(value: &'a token::Comma) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a token::Const> for Node {
    fn from(value: &'a token::Const) -> Self {
        span_only(value, "Const")
    }
}

impl<'a> From<&'a token::Const> for Value {
    fn from(value: &'a token::Const) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a token::Eq> for Node {
    fn from(value: &'a token::Eq) -> Self {
        span_only(value, "Eq")
    }
}

impl<'a> From<&'a token::Eq> for Value {
    fn from(value: &'a token::Eq) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a token::Fn> for Node {
    fn from(value: &'a token::Fn) -> Self {
        span_only(value, "Fn")
    }
}

impl<'a> From<&'a token::Fn> for Value {
    fn from(value: &'a token::Fn) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a token::For> for Node {
    fn from(value: &'a token::For) -> Self {
        span_only(value, "For")
    }
}

impl<'a> From<&'a token::For> for Value {
    fn from(value: &'a token::For) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a token::Gt> for Node {
    fn from(value: &'a token::Gt) -> Self {
        span_only(value, "Gt")
    }
}

impl<'a> From<&'a token::Gt> for Value {
    fn from(value: &'a token::Gt) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a token::Lt> for Node {
    fn from(value: &'a token::Lt) -> Self {
        span_only(value, "Lt")
    }
}

impl<'a> From<&'a token::Lt> for Value {
    fn from(value: &'a token::Lt) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a token::Mut> for Node {
    fn from(value: &'a token::Mut) -> Self {
        span_only(value, "Mut")
    }
}

impl<'a> From<&'a token::Mut> for Value {
    fn from(value: &'a token::Mut) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a token::Paren> for Node {
    fn from(value: &'a token::Paren) -> Self {
        delim_span_only(&value.span, "Paren")
    }
}

impl<'a> From<&'a token::Paren> for Value {
    fn from(value: &'a token::Paren) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a token::PathSep> for Node {
    fn from(value: &'a token::PathSep) -> Self {
        span_only(value, "PathSep")
    }
}

impl<'a> From<&'a token::PathSep> for Value {
    fn from(value: &'a token::PathSep) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a token::Pub> for Node {
    fn from(value: &'a token::Pub) -> Self {
        span_only(value, "Pub")
    }
}

impl<'a> From<&'a token::Pub> for Value {
    fn from(value: &'a token::Pub) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a token::Semi> for Node {
    fn from(value: &'a token::Semi) -> Self {
        span_only(value, "Semi")
    }
}

impl<'a> From<&'a token::Semi> for Value {
    fn from(value: &'a token::Semi) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a token::Unsafe> for Node {
    fn from(value: &'a token::Unsafe) -> Self {
        span_only(value, "Unsafe")
    }
}

impl<'a> From<&'a token::Unsafe> for Value {
    fn from(value: &'a token::Unsafe) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a Ident> for Node {
    fn from(value: &'a Ident) -> Self {
        Self::new(
            "Ident".to_smolstr(),
            Some(value.span().into()),
            vec![
                NodeChild::new("to_string".to_smolstr(), value.to_string().into()),
                span_child(value),
            ],
        )
    }
}

impl<'a> From<&'a Ident> for Value {
    fn from(value: &'a Ident) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a Generics> for Node {
    fn from(value: &'a Generics) -> Self {
        Self::new(
            "Generics".to_smolstr(),
            Some(value.span().into()),
            vec![
                NodeChild::new("lt_token".to_smolstr(), from_option(&value.lt_token)),
                NodeChild::new("params".to_smolstr(), (&value.params).into()),
                NodeChild::new("gt_token".to_smolstr(), from_option(&value.gt_token)),
                NodeChild::new(
                    "where_clause".to_smolstr(),
                    from_option(&value.where_clause),
                ),
                span_child(value),
            ],
        )
    }
}

impl<'a> From<&'a Generics> for Value {
    fn from(value: &'a Generics) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a Type> for Node {
    fn from(value: &'a Type) -> Self {
        match value {
            Type::Reference(type_) => type_.into(),
            Type::Slice(type_) => type_.into(),
            Type::Path(type_) => type_.into(),
            _ => unimplemented!(),
        }
    }
}

impl<'a> From<&'a Type> for Value {
    fn from(value: &'a Type) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a TypeReference> for Node {
    fn from(value: &'a TypeReference) -> Self {
        Self::new(
            "TypeReference".to_smolstr(),
            Some(value.span().into()),
            vec![
                NodeChild::new("and_token".to_smolstr(), (&value.and_token).into()),
                NodeChild::new("lifetime".to_smolstr(), from_option(&value.lifetime)),
                NodeChild::new("mutability".to_smolstr(), from_option(&value.mutability)),
                NodeChild::new("elem".to_smolstr(), (&*value.elem).into()),
                span_child(value),
            ],
        )
    }
}

impl<'a> From<&'a TypeReference> for Value {
    fn from(value: &'a TypeReference) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a TypeSlice> for Node {
    fn from(value: &'a TypeSlice) -> Self {
        Self::new(
            "TypeSlice".to_smolstr(),
            Some(value.span().into()),
            vec![
                NodeChild::new("bracket_token".to_smolstr(), (&value.bracket_token).into()),
                NodeChild::new("elem".to_smolstr(), (&*value.elem).into()),
                span_child(value),
            ],
        )
    }
}

impl<'a> From<&'a TypeSlice> for Value {
    fn from(value: &'a TypeSlice) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a TypePath> for Node {
    fn from(value: &'a TypePath) -> Self {
        Self::new(
            "TypePath".to_smolstr(),
            Some(value.span().into()),
            vec![
                NodeChild::new("qself".to_smolstr(), from_option(&value.qself)),
                NodeChild::new("path".to_smolstr(), (&value.path).into()),
                span_child(value),
            ],
        )
    }
}

impl<'a> From<&'a TypePath> for Value {
    fn from(value: &'a TypePath) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a Expr> for Node {
    fn from(value: &'a Expr) -> Self {
        match value {
            Expr::Reference(expr) => expr.into(),
            Expr::Array(expr) => expr.into(),
            Expr::Lit(expr) => expr.into(),
            Expr::ForLoop(expr) => expr.into(),
            _ => unimplemented!(),
        }
    }
}

impl<'a> From<&'a Expr> for Value {
    fn from(value: &'a Expr) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a ExprReference> for Node {
    fn from(value: &'a ExprReference) -> Self {
        Self::new(
            "ExprReference".to_smolstr(),
            Some(value.span().into()),
            vec![
                NodeChild::new("attrs".to_smolstr(), from_slice(&value.attrs)),
                NodeChild::new("and_token".to_smolstr(), (&value.and_token).into()),
                NodeChild::new("mutability".to_smolstr(), from_option(&value.mutability)),
                NodeChild::new("expr".to_smolstr(), (&*value.expr).into()),
                span_child(value),
            ],
        )
    }
}

impl<'a> From<&'a ExprReference> for Value {
    fn from(value: &'a ExprReference) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a ExprArray> for Node {
    fn from(value: &'a ExprArray) -> Self {
        Self::new(
            "ExprArray".to_smolstr(),
            Some(value.span().into()),
            vec![
                NodeChild::new("attrs".to_smolstr(), from_slice(&value.attrs)),
                NodeChild::new("bracket_token".to_smolstr(), (&value.bracket_token).into()),
                NodeChild::new("elems".to_smolstr(), (&value.elems).into()),
                span_child(value),
            ],
        )
    }
}

impl<'a> From<&'a ExprArray> for Value {
    fn from(value: &'a ExprArray) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a ExprLit> for Node {
    fn from(value: &'a ExprLit) -> Self {
        Self::new(
            "ExprLit".to_smolstr(),
            Some(value.span().into()),
            vec![
                NodeChild::new("attrs".to_smolstr(), from_slice(&value.attrs)),
                NodeChild::new("lit".to_smolstr(), (&value.lit).into()),
                span_child(value),
            ],
        )
    }
}

impl<'a> From<&'a ExprLit> for Value {
    fn from(value: &'a ExprLit) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a ExprForLoop> for Node {
    fn from(value: &'a ExprForLoop) -> Self {
        Self::new(
            "ExprForLoop".to_smolstr(),
            Some(value.span().into()),
            vec![
                NodeChild::new("attrs".to_smolstr(), from_slice(&value.attrs)),
                NodeChild::new("label".to_smolstr(), from_option(&value.label)),
                NodeChild::new("for_token".to_smolstr(), (&value.for_token).into()),
                NodeChild::new("pat".to_smolstr(), (&*value.pat).into()),
                span_child(value),
            ],
        )
    }
}

impl<'a> From<&'a ExprForLoop> for Value {
    fn from(value: &'a ExprForLoop) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a Signature> for Node {
    fn from(value: &'a Signature) -> Self {
        Self::new(
            "Signature".to_smolstr(),
            Some(value.span().into()),
            vec![
                NodeChild::new("constness".to_smolstr(), from_option(&value.constness)),
                NodeChild::new("asyncness".to_smolstr(), from_option(&value.asyncness)),
                NodeChild::new("unsafety".to_smolstr(), from_option(&value.unsafety)),
                NodeChild::new("abi".to_smolstr(), from_option(&value.abi)),
                NodeChild::new("fn_token".to_smolstr(), (&value.fn_token).into()),
                NodeChild::new("ident".to_smolstr(), (&value.ident).into()),
                NodeChild::new("generics".to_smolstr(), (&value.generics).into()),
                NodeChild::new("paren_token".to_smolstr(), (&value.paren_token).into()),
                NodeChild::new("inputs".to_smolstr(), (&value.inputs).into()),
                NodeChild::new("variadic".to_smolstr(), from_option(&value.variadic)),
                NodeChild::new("output".to_smolstr(), (&value.output).into()),
                span_child(value),
            ],
        )
    }
}

impl<'a> From<&'a Signature> for Value {
    fn from(value: &'a Signature) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a Block> for Node {
    fn from(value: &'a Block) -> Self {
        Self::new(
            "Block".to_smolstr(),
            Some(value.span().into()),
            vec![
                NodeChild::new("brace_token".to_smolstr(), (&value.brace_token).into()),
                NodeChild::new("stmts".to_smolstr(), from_slice(&value.stmts)),
                span_child(value),
            ],
        )
    }
}

impl<'a> From<&'a Block> for Value {
    fn from(value: &'a Block) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a GenericParam> for Node {
    fn from(value: &'a GenericParam) -> Self {
        unimplemented!()
    }
}

impl<'a> From<&'a GenericParam> for Value {
    fn from(value: &'a GenericParam) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a WhereClause> for Node {
    fn from(value: &'a WhereClause) -> Self {
        unimplemented!()
    }
}

impl<'a> From<&'a WhereClause> for Value {
    fn from(value: &'a WhereClause) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a Lifetime> for Node {
    fn from(value: &'a Lifetime) -> Self {
        unimplemented!()
    }
}

impl<'a> From<&'a Lifetime> for Value {
    fn from(value: &'a Lifetime) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a QSelf> for Node {
    fn from(value: &'a QSelf) -> Self {
        unimplemented!()
    }
}

impl<'a> From<&'a QSelf> for Value {
    fn from(value: &'a QSelf) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a Path> for Node {
    fn from(value: &'a Path) -> Self {
        Self::new(
            "Path".to_smolstr(),
            Some(value.span().into()),
            vec![
                NodeChild::new(
                    "leading_colon".to_smolstr(),
                    from_option(&value.leading_colon),
                ),
                NodeChild::new("segments".to_smolstr(), (&value.segments).into()),
                span_child(value),
            ],
        )
    }
}

impl<'a> From<&'a Path> for Value {
    fn from(value: &'a Path) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a PathSegment> for Node {
    fn from(value: &'a PathSegment) -> Self {
        Self::new(
            "PathSegment".to_smolstr(),
            Some(value.span().into()),
            vec![
                NodeChild::new("ident".to_smolstr(), (&value.ident).into()),
                NodeChild::new("arguments".to_smolstr(), (&value.arguments).into()),
                span_child(value),
            ],
        )
    }
}

impl<'a> From<&'a PathSegment> for Value {
    fn from(value: &'a PathSegment) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a PathArguments> for Value {
    fn from(value: &'a PathArguments) -> Self {
        match value {
            PathArguments::None => Self::Scalar("PathArguments::None".to_smolstr()),
            _ => unimplemented!(),
        }
    }
}

impl<'a> From<&'a Lit> for Node {
    fn from(value: &'a Lit) -> Self {
        match value {
            Lit::Str(lit) => lit.into(),
            _ => unimplemented!(),
        }
    }
}

impl<'a> From<&'a Lit> for Value {
    fn from(value: &'a Lit) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a LitStr> for Node {
    fn from(value: &'a LitStr) -> Self {
        Self::new(
            "LitStr".to_smolstr(),
            Some(value.span().into()),
            vec![
                NodeChild::new("value".to_smolstr(), value.value().into()),
                NodeChild::new("suffix".to_smolstr(), value.suffix().into()),
                span_child(value),
            ],
        )
    }
}

impl<'a> From<&'a LitStr> for Value {
    fn from(value: &'a LitStr) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a Abi> for Node {
    fn from(value: &'a Abi) -> Self {
        unimplemented!()
    }
}

impl<'a> From<&'a Abi> for Value {
    fn from(value: &'a Abi) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a FnArg> for Node {
    fn from(value: &'a FnArg) -> Self {
        unimplemented!()
    }
}

impl<'a> From<&'a FnArg> for Value {
    fn from(value: &'a FnArg) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a Variadic> for Node {
    fn from(value: &'a Variadic) -> Self {
        unimplemented!()
    }
}

impl<'a> From<&'a Variadic> for Value {
    fn from(value: &'a Variadic) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a ReturnType> for Value {
    fn from(value: &'a ReturnType) -> Self {
        match value {
            ReturnType::Default => Self::Scalar("Default".to_smolstr()),
            _ => unimplemented!(),
        }
    }
}

impl<'a> From<&'a Stmt> for Node {
    fn from(value: &'a Stmt) -> Self {
        match value {
            Stmt::Expr(stmt, semi) => match semi {
                None => stmt.into(),
                Some(semi) => Self::new(
                    "Expr".to_smolstr(),
                    Some(value.span().into()),
                    vec![
                        NodeChild::new("expr".to_smolstr(), stmt.into()),
                        NodeChild::new("semi".to_smolstr(), semi.into()),
                        span_child(value),
                    ],
                ),
            },
            _ => unimplemented!(),
        }
    }
}

impl<'a> From<&'a Stmt> for Value {
    fn from(value: &'a Stmt) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a Label> for Node {
    fn from(value: &'a Label) -> Self {
        unimplemented!()
    }
}

impl<'a> From<&'a Label> for Value {
    fn from(value: &'a Label) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a Pat> for Node {
    fn from(value: &'a Pat) -> Self {
        unimplemented!()
    }
}

impl<'a> From<&'a Pat> for Value {
    fn from(value: &'a Pat) -> Self {
        Node::from(value).into()
    }
}

impl<'a, TItem, TPunctuation> From<&'a Punctuated<TItem, TPunctuation>> for Value
where
    for<'b> &'b TItem: Into<Node>,
    for<'b> &'b TPunctuation: Into<Node>,
{
    fn from(value: &'a Punctuated<TItem, TPunctuation>) -> Self {
        value
            .pairs()
            .flat_map(|pair| -> SmallVec<_, 10> {
                match pair {
                    punctuated::Pair::Punctuated(item, punctuation) => {
                        smallvec![item.into(), punctuation.into(),]
                    }
                    punctuated::Pair::End(item) => smallvec![item.into(),],
                }
            })
            .collect::<Nodes>()
            .into()
    }
}

impl From<Span> for Range {
    fn from(value: Span) -> Self {
        Self {
            start: Location::OffsetAndPosition {
                offset: value.byte_range().start,
                position: Position {
                    line: value.start().line - 1,
                    column: value.start().column,
                },
            },
            end: Location::OffsetAndPosition {
                offset: value.byte_range().end,
                position: Position {
                    line: value.end().line - 1,
                    column: value.end().column,
                },
            },
        }
    }
}

fn from_option_string(value: &Option<String>) -> Value {
    match value.as_ref() {
        None => "None".into(),
        Some(value) => value.into(),
    }
}

fn from_slice<TItem>(list: &[TItem]) -> Value
where
    for<'a> &'a TItem: Into<Node>,
{
    list.into_iter().map(Into::into).collect::<Nodes>().into()
}

fn span_child<TSpanned: Spanned>(value: &TSpanned) -> NodeChild {
    NodeChild::new("span".to_smolstr(), from_span(&value.span()))
}

fn span_only<TSpanned: Spanned>(value: &TSpanned, name: &str) -> Node {
    Node::new(name.to_smolstr(), None, vec![span_child(value)])
}

fn delim_span_only(value: &DelimSpan, name: &str) -> Node {
    Node::new(
        name.to_smolstr(),
        None,
        vec![
            NodeChild::new("open".to_smolstr(), from_span(&value.open())),
            NodeChild::new("close".to_smolstr(), from_span(&value.close())),
        ],
    )
}

fn from_span(value: &Span) -> Value {
    Value::Node(Node::new(
        "Span".to_smolstr(),
        None,
        vec![
            NodeChild::new("start".to_smolstr(), from_line_column(&value.start())),
            NodeChild::new("end".to_smolstr(), from_line_column(&value.end())),
        ],
    ))
}

fn from_line_column(value: &LineColumn) -> Value {
    Value::Node(Node::new(
        "LineColumn".to_smolstr(),
        None,
        vec![
            NodeChild::new("line".to_smolstr(), format!("{}", value.line + 1).into()),
            NodeChild::new("column".to_smolstr(), format!("{}", value.column).into()),
        ],
    ))
}

fn from_option<TValue>(value: &Option<TValue>) -> Value
where
    for<'a> &'a TValue: Into<Value>,
{
    match value.as_ref() {
        None => Value::Scalar("None".to_smolstr()),
        Some(value) => value.into(),
    }
}

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }
}

impl Parse for Parser {
    fn parse(&self, text: &str) -> Result<Node, Error> {
        Ok(Node::from(
            &syn::parse_file(text).map_err(|err| Error::Parse(err.to_smolstr()))?,
        ))
    }
}
