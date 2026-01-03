use oelung::{anyhow, soft, Component, ComponentInterface, Grid};
use smallvec::SmallVec;
use smol_str::{SmolStr, SmolStrBuilder};

use crate::ast;

const SPACES_PER_NESTING_LEVEL: usize = 4;

pub struct AstPanel {
    pub tree: ast::Node,
}

impl<'a> ComponentInterface for &'a AstPanel {
    fn render(&self, _grid: Grid) -> Result<Component<'_>, anyhow::Error> {
        Ok(soft! {
            %Node::new(&self.tree, 0)
        })
    }
}

pub struct Node<'a> {
    pub node: &'a ast::Node,
    pub nesting_level: usize,
}

impl<'a> Node<'a> {
    pub fn new(node: &'a ast::Node, nesting_level: usize) -> Self {
        Self {
            node,
            nesting_level,
        }
    }
}

impl<'a> ComponentInterface for Node<'a> {
    fn render(&self, _grid: Grid) -> Result<Component<'_>, anyhow::Error> {
        Ok(soft! {
            %FlexColumn
                children => {
                    [
                        soft! {
                            %Text
                              children => [
                                %Text &self.node.type_
                                %Text " {"
                              ]
                        },
                    ].into_iter()
                        .chain(
                            self.node.children.iter().map(|child| -> Result<_, anyhow::Error> {
                                Ok(soft! {
                                    %NodeChild::new(child, self.nesting_level)
                                })
                            }).collect::<Result<SmallVec<_, 10>, _>>()?
                        )
                        .chain(
                            [soft! {
                                %Text children => [
                                  %InitialSpaces::new(self.nesting_level)
                                  %Text "}"
                                ]
                            }]
                        )
                        .collect()
                }
                flex_grow => 1
        })
    }
}

pub struct InitialSpaces {
    pub text: SmolStr,
}

impl InitialSpaces {
    pub fn new(nesting_level: usize) -> Self {
        Self {
            text: {
                let mut text = SmolStrBuilder::default();
                for _ in 0..nesting_level {
                    for _ in 0..SPACES_PER_NESTING_LEVEL {
                        text.push(' ');
                    }
                }
                text.finish()
            },
        }
    }
}

impl ComponentInterface for InitialSpaces {
    fn render(&self, _grid: Grid) -> Result<Component<'_>, anyhow::Error> {
        Ok(soft! {
            %Text &self.text
        })
    }
}

pub struct NodeChild<'a> {
    pub node_child: &'a ast::NodeChild,
    pub nesting_level: usize,
}

impl<'a> NodeChild<'a> {
    pub fn new(node_child: &'a ast::NodeChild, nesting_level: usize) -> Self {
        Self {
            node_child,
            nesting_level,
        }
    }
}

impl<'a> ComponentInterface for NodeChild<'a> {
    fn render(&self, _grid: Grid) -> Result<Component<'_>, anyhow::Error> {
        Ok(match &self.node_child.value {
            ast::Value::Scalar(value) => soft! {
                %Text children => [
                  %NodeChildName::new(&self.node_child.name)
                  %Text " "
                  %Text value
                ]
            },
            ast::Value::Array(list) => match list.is_empty() {
                true => soft! {
                    %Text children => [
                      %NodeChildName::new(&self.node_child.name)
                      %Text " []"
                    ]
                },
                false => unimplemented!(),
            },
            _ => unimplemented!(),
        })
    }
}

pub struct NodeChildName<'a> {
    pub name: &'a str,
}

impl<'a> NodeChildName<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }
}

impl<'a> ComponentInterface for NodeChildName<'a> {
    fn render(&self, _grid: Grid) -> Result<Component<'_>, anyhow::Error> {
        Ok(soft! {
            %Text children => [
              %Text self.name
              %Text ":"
            ]
        })
    }
}
