use smallvec::SmallVec;
use smol_str::{SmolStr, ToSmolStr};

use crate::Error;

pub enum Value {
    Node(Node),
    Array(Nodes),
    Scalar(SmolStr),
}

impl From<Node> for Value {
    fn from(value: Node) -> Self {
        Self::Node(value)
    }
}

impl From<Nodes> for Value {
    fn from(value: Nodes) -> Self {
        Self::Array(value)
    }
}

impl<'a> From<&'a str> for Value {
    fn from(value: &'a str) -> Self {
        Self::Scalar(value.to_smolstr())
    }
}

impl<'a> From<&'a String> for Value {
    fn from(value: &'a String) -> Self {
        Self::Scalar(value.to_smolstr())
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::Scalar(value.to_smolstr())
    }
}

pub type Nodes = SmallVec<Node, 10>;

pub struct Node {
    pub type_: SmolStr,
    pub range: Option<Range>,
    pub children: Vec<NodeChild>,
}

impl Node {
    pub fn new(type_: SmolStr, range: Option<Range>, children: Vec<NodeChild>) -> Self {
        Self {
            type_,
            range,
            children,
        }
    }
}

pub struct NodeChild {
    pub name: SmolStr,
    pub value: Value,
}

impl NodeChild {
    pub fn new(name: SmolStr, value: Value) -> Self {
        Self { name, value }
    }
}

pub type Offset = usize;

#[derive(Copy, Clone, Debug)]
pub struct Range {
    pub start: Location,
    pub end: Location,
}

#[derive(Copy, Clone, Debug)]
pub enum Location {
    // JustOffset(Offset),
    OffsetAndPosition { offset: Offset, position: Position },
    // JustPosition(Position),
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
    /// 0-based
    pub line: usize,
    /// 0-based
    pub column: usize,
}

pub trait Parse {
    fn parse(&self, text: &str) -> Result<Node, Error>;
}
