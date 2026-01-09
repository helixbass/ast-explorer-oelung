use smol_str::ToSmolStr;

use crate::{Error, Location, Node, NodeChild, Parse, Position, Range, Value};

impl<'a> From<tree_sitter::Node<'a>> for Node {
    fn from(value: tree_sitter::Node<'a>) -> Self {
        let mut cursor = value.walk();
        Self::new(
            value.kind().to_smolstr(),
            Some(Range {
                start: Location::OffsetAndPosition {
                    offset: value.start_byte(),
                    position: Position {
                        line: value.start_position().row,
                        column: value.start_position().column,
                    },
                },
                end: Location::OffsetAndPosition {
                    offset: value.end_byte(),
                    position: Position {
                        line: value.end_position().row,
                        column: value.end_position().column,
                    },
                },
            }),
            value
                .children(&mut cursor)
                .enumerate()
                .map(|(child_index, child)| {
                    NodeChild::new(
                        value
                            .field_name_for_child(u32::try_from(child_index).unwrap())
                            .unwrap_or(child.kind())
                            .to_smolstr(),
                        child.into(),
                    )
                })
                .collect(),
            false,
        )
    }
}

impl<'a> From<tree_sitter::Node<'a>> for Value {
    fn from(value: tree_sitter::Node<'a>) -> Self {
        Node::from(value).into()
    }
}

pub struct Parser {
    parser: tree_sitter::Parser,
}

impl Parser {
    pub fn new(parser: tree_sitter::Parser) -> Self {
        assert!(parser.language().is_some());
        Self { parser }
    }
}

impl Parse for Parser {
    fn parse(&mut self, text: &str) -> Result<Node, Error> {
        Ok(Node::from(
            self.parser.parse(text, None).unwrap().root_node(),
        ))
    }
}
