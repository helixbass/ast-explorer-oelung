use smallvec::SmallVec;
use smol_str::{SmolStr, ToSmolStr};

use crate::Error;

#[derive(Debug)]
pub enum Value {
    Node(Node),
    Array(Nodes),
    Scalar(SmolStr),
}

impl Value {
    pub fn as_node(&self) -> &Node {
        match self {
            Self::Node(node) => node,
            _ => panic!("expected node"),
        }
    }

    pub fn get_path(&self, path: &[NodePathStep]) -> &Value {
        if path.is_empty() {
            self
        } else {
            match (self, path[0]) {
                (Self::Node(node), NodePathStep::NodeChild(_)) => node.get_path(path),
                (Self::Array(nodes), NodePathStep::ArrayChild(index)) => {
                    nodes[index].get_path(&path[1..])
                }
                _ => panic!("node path didn't match up"),
            }
        }
    }
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

#[derive(Debug)]
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

    pub fn get_path(&self, path: &[NodePathStep]) -> &Value {
        assert!(!path.is_empty());
        match path[0] {
            NodePathStep::NodeChild(index) => {
                let child = &self.children[index];
                if path.len() == 1 {
                    return &child.value;
                }
                child.value.get_path(&path[1..])
            }
            _ => panic!("node path didn't match up"),
        }
    }
}

#[derive(Debug)]
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

pub type NodePath = SmallVec<NodePathStep, 10>;

#[derive(Copy, Clone, Debug)]
pub enum NodePathStep {
    NodeChild(usize),
    ArrayChild(usize),
}
