use proc_macro2::{LineColumn, Span};
use smallvec::{smallvec, SmallVec};
use smol_str::ToSmolStr;
use syn::{
    punctuated::{self, Punctuated},
    spanned::Spanned,
    token, Attribute, Block, Expr, ExprReference, File, GenericParam, Generics, Ident, Item,
    ItemConst, ItemFn, Signature, Type, TypeReference, Visibility, WhereClause,
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
        unimplemented!()
    }
}

impl<'a> From<&'a TypeReference> for Value {
    fn from(value: &'a TypeReference) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a Expr> for Node {
    fn from(value: &'a Expr) -> Self {
        match value {
            Expr::Reference(expr) => expr.into(),
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
        unimplemented!()
    }
}

impl<'a> From<&'a ExprReference> for Value {
    fn from(value: &'a ExprReference) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a Signature> for Node {
    fn from(value: &'a Signature) -> Self {
        unimplemented!()
    }
}

impl<'a> From<&'a Signature> for Value {
    fn from(value: &'a Signature) -> Self {
        Node::from(value).into()
    }
}

impl<'a> From<&'a Block> for Node {
    fn from(value: &'a Block) -> Self {
        unimplemented!()
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
