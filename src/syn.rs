use proc_macro2::{LineColumn, Span};
use smol_str::ToSmolStr;
use syn::{spanned::Spanned, token, Attribute, File, Ident, Item, ItemConst, Visibility};

use crate::{Location, Node, NodeChild, Position, Range, Value};

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

impl<'a> From<&'a Visibility> for Node {
    fn from(value: &'a Visibility) -> Self {
        unimplemented!()
    }
}

impl<'a> From<&'a Visibility> for Value {
    fn from(value: &'a Visibility) -> Self {
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
    for<'a> &'a TItem: Into<Value>,
{
    list.into_iter().map(Into::into).collect::<Vec<_>>().into()
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
