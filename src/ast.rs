use std::cmp::Ordering;

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

    pub fn get_path(&self, path: &[NodePathStep]) -> ValueOrNode<'_> {
        if path.is_empty() {
            self.into()
        } else {
            match (self, path[0]) {
                (Self::Node(node), NodePathStep::NodeChild(_)) => node.get_path(path),
                (Self::Array(nodes), NodePathStep::ArrayChild(index)) => {
                    let node = &nodes[index];
                    if path.len() == 1 {
                        return node.into();
                    }
                    node.get_path(&path[1..])
                }
                _ => panic!("node path didn't match up"),
            }
        }
    }

    pub fn get_path_of_smallest_containing_node(
        &self,
        position: Position,
        path: NodePath,
    ) -> Option<NodePath> {
        match self {
            Self::Node(node) => node.get_path_of_smallest_containing_node(position, path),
            Self::Array(nodes) => {
                nodes
                    .into_iter()
                    .enumerate()
                    .find_map(|(array_index, array_child)| {
                        array_child.get_path_of_smallest_containing_node(
                            position,
                            node_path_appended(&path, NodePathStep::ArrayChild(array_index)),
                        )
                    })
            }
            _ => None,
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

impl From<char> for Value {
    fn from(value: char) -> Self {
        Self::Scalar(value.to_smolstr())
    }
}

pub type Nodes = SmallVec<Node, 10>;

#[derive(Debug)]
pub struct Node {
    pub type_: SmolStr,
    pub range: Option<Range>,
    pub children: Vec<NodeChild>,
    pub is_location: bool,
}

impl Node {
    pub fn new(
        type_: SmolStr,
        range: Option<Range>,
        children: Vec<NodeChild>,
        is_location: bool,
    ) -> Self {
        Self {
            type_,
            range,
            children,
            is_location,
        }
    }

    pub fn get_path(&self, path: &[NodePathStep]) -> ValueOrNode<'_> {
        assert!(!path.is_empty());
        match path[0] {
            NodePathStep::NodeChild(index) => {
                let child = &self.children[index];
                if path.len() == 1 {
                    return (&child.value).into();
                }
                child.value.get_path(&path[1..])
            }
            _ => panic!("node path didn't match up"),
        }
    }

    pub fn get_path_of_smallest_containing_node(
        &self,
        position: Position,
        path: NodePath,
    ) -> Option<NodePath> {
        match self.range?.contains(position) {
            false => None,
            true => Some(
                self.children
                    .iter()
                    .enumerate()
                    .find_map(|(child_index, child)| {
                        child.value.get_path_of_smallest_containing_node(
                            position,
                            node_path_appended(&path, NodePathStep::NodeChild(child_index)),
                        )
                    })
                    .unwrap_or(path),
            ),
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

impl Range {
    pub fn contains(&self, position: Position) -> bool {
        match self.start {
            Location::OffsetAndPosition {
                position: start_position,
                ..
            } if start_position <= position => match self.end {
                Location::OffsetAndPosition {
                    position: end_position,
                    ..
                } if position < end_position => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn overlaps_with_line_num(&self, line_num: usize) -> bool {
        match self.start {
            Location::OffsetAndPosition {
                position: start_position,
                ..
            } if start_position.line <= line_num => match self.end {
                Location::OffsetAndPosition {
                    position: end_position,
                    ..
                } if (line_num < end_position.line)
                    || (line_num == end_position.line && end_position.column != 0) =>
                {
                    true
                }
                _ => false,
            },
            _ => false,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Location {
    // JustOffset(Offset),
    OffsetAndPosition { offset: Offset, position: Position },
    // JustPosition(Position),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Position {
    /// 0-based
    pub line: usize,
    /// 0-based
    pub column: usize,
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Position) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Position) -> Ordering {
        match self.line.cmp(&other.line) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self.column.cmp(&other.column),
        }
    }
}

pub trait Parse {
    fn parse(&mut self, text: &str) -> Result<Node, Error>;
}

pub type NodePath = SmallVec<NodePathStep, 10>;

pub fn node_path_appended(path: &NodePath, step: NodePathStep) -> NodePath {
    let mut path = path.clone();
    path.push(step);
    path
}

pub fn node_path_parent_node(path: &NodePath) -> Option<NodePath> {
    if path.len() == 1 {
        return None;
    }
    let mut path = path.clone();
    let mut just_saw_array_child = matches!(path[path.len() - 1], NodePathStep::ArrayChild(_));
    for index in (0..path.len() - 1).rev() {
        if !just_saw_array_child {
            path.truncate(index + 1);
            return Some(path);
        }
        just_saw_array_child = matches!(path[index], NodePathStep::ArrayChild(_));
    }
    None
}

#[derive(Copy, Clone, Debug)]
pub enum NodePathStep {
    NodeChild(usize),
    ArrayChild(usize),
}

pub enum ValueOrNode<'a> {
    Value(&'a Value),
    Node(&'a Node),
}

impl<'a> ValueOrNode<'a> {
    pub fn as_node(&self) -> &'a Node {
        match self {
            Self::Value(value) => value.as_node(),
            Self::Node(node) => node,
        }
    }
}

impl<'a> From<&'a Node> for ValueOrNode<'a> {
    fn from(value: &'a Node) -> Self {
        Self::Node(value)
    }
}

impl<'a> From<&'a Value> for ValueOrNode<'a> {
    fn from(value: &'a Value) -> Self {
        Self::Value(value)
    }
}
