use oelung::{anyhow, soft, Component, ComponentInterface, Grid};
use smallvec::SmallVec;
use smol_str::{SmolStr, SmolStrBuilder};
use squalid::BoolExt;

use crate::ast;

const SPACES_PER_NESTING_LEVEL: usize = 4;

pub struct AstPanel<'a> {
    pub tree: &'a ast::Node,
}

impl<'a> AstPanel<'a> {
    pub fn new(tree: &'a ast::Node) -> Self {
        Self { tree }
    }
}

impl<'a> ComponentInterface for AstPanel<'a> {
    fn render(&self, _grid: Grid) -> Result<Component<'_>, anyhow::Error> {
        Ok(soft! {
            %FlexColumn
              children => [
                %Node::new(self.tree, 0, true)
              ]
              overflow_y => hidden
        })
    }
}

pub struct Node<'a> {
    pub node: &'a ast::Node,
    pub nesting_level: usize,
    pub should_print_top_and_bottom_lines: bool,
}

impl<'a> Node<'a> {
    pub fn new(
        node: &'a ast::Node,
        nesting_level: usize,
        should_print_top_and_bottom_lines: bool,
    ) -> Self {
        Self {
            node,
            nesting_level,
            should_print_top_and_bottom_lines,
        }
    }
}

impl<'a> ComponentInterface for Node<'a> {
    fn render(&self, _grid: Grid) -> Result<Component<'_>, anyhow::Error> {
        Ok(soft! {
            %FlexColumn
                children => {
                    self.should_print_top_and_bottom_lines.try_then(|| -> Result<_, anyhow::Error> {
                        Ok(soft! {
                            %Text
                              children => [
                                %InitialSpaces::new(self.nesting_level)
                                %Text &self.node.type_
                                %Text " {"
                              ]
                        })
                    })?.into_iter()
                        .chain(
                            self.node.children.iter().map(|child| -> Result<_, anyhow::Error> {
                                Ok(soft! {
                                    %NodeChild::new(child, self.nesting_level)
                                })
                            }).collect::<Result<SmallVec<_, 10>, _>>()?
                        )
                        .chain(
                            self.should_print_top_and_bottom_lines.try_then(|| -> Result<_, anyhow::Error> {
                                Ok(soft! {
                                    %Text
                                      children => [
                                        %InitialSpaces::new(self.nesting_level)
                                        %Text "}"
                                      ]
                                })
                            })?.into_iter()
                        )
                        .collect()
                }
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
                      %InitialSpaces::new(self.nesting_level + 1)
                      %NodeChildName::new(&self.node_child.name)
                      %Text " []"
                    ]
                },
                false => soft! {
                    %FlexColumn children => [
                        %Text children => [
                          %InitialSpaces::new(self.nesting_level + 1)
                          %NodeChildName::new(&self.node_child.name)
                          %Text " ["
                        ]
                        %Array::new(list, self.nesting_level)
                        %Text children => [
                          %InitialSpaces::new(self.nesting_level + 1)
                          %Text "]"
                        ]

                    ]
                },
            },
            ast::Value::Node(node) => soft! {
                %FlexColumn children => [
                  %Text children => [
                    %InitialSpaces::new(self.nesting_level + 1)
                    %NodeChildName::new(&self.node_child.name)
                    %Text " "
                    %Text &node.type_
                    %Text " {"
                  ]
                  %Node::new(node, self.nesting_level + 1, false)
                  %Text children => [
                    %InitialSpaces::new(self.nesting_level + 1)
                    %Text "}"
                  ]
                ]
            },
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

pub struct Array<'a> {
    pub nodes: &'a [ast::Node],
    pub nesting_level: usize,
}

impl<'a> Array<'a> {
    pub fn new(nodes: &'a [ast::Node], nesting_level: usize) -> Self {
        assert!(!nodes.is_empty());
        Self {
            nodes,
            nesting_level,
        }
    }
}

impl<'a> ComponentInterface for Array<'a> {
    fn render(&self, _grid: Grid) -> Result<Component<'_>, anyhow::Error> {
        Ok(soft! {
            %FlexColumn
              children => self.nodes.into_iter().map(|node| -> Result<_, anyhow::Error> {
                  Ok(soft! {
                      %Node::new(node, self.nesting_level + 2, true)
                  })
              }).collect::<Result<Vec<_>, _>>()?
        })
    }
}
