use proc_macro2::Span;
use smol_str::ToSmolStr;
use syn::File;

use crate::{Location, Node, Position, Range};

impl<'a> From<&'a File> for Node {
    fn from(value: &'a File) -> Self {
        Self::new("File".to_smolstr(), Some(value.span().into()))
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
