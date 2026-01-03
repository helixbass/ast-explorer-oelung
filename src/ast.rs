use smol_str::SmolStr;

pub enum Value {
    Node(Node),
    Array(Vec<Value>),
    Scalar(SmolStr),
}

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
