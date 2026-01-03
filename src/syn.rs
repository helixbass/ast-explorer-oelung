use proc_macro2::Span;
use smol_str::ToSmolStr;
use syn::{spanned::Spanned, Attribute, File};

use crate::{Location, Node, NodeChild, Position, Range, Value};

impl<'a> From<&'a File> for Node {
    fn from(value: &'a File) -> Self {
        Self::new(
            "File".to_smolstr(),
            Some(value.span().into()),
            vec![
                NodeChild::new("shebang".to_smolstr(), from_option_string(&value.shebang)),
                NodeChild::new("attrs".to_smolstr(), from_slice(&value.attrs)),
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
