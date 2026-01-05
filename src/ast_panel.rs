use oelung::{anyhow, soft, Component, ComponentInterface, Grid};
use smallvec::SmallVec;
use smol_str::{SmolStr, SmolStrBuilder};
use squalid::BoolExt;

use crate::{ast, NodePath};

const SPACES_PER_NESTING_LEVEL: usize = 2;

pub struct AstPanel<'a> {
    pub tree: &'a ast::Node,
    pub current_zoomed_node: Option<&'a NodePath>,
    pub are_locations_expanded: bool,
}

impl<'a> AstPanel<'a> {
    pub fn new(
        tree: &'a ast::Node,
        current_zoomed_node: Option<&'a NodePath>,
        are_locations_expanded: bool,
    ) -> Self {
        Self {
            tree,
            current_zoomed_node,
            are_locations_expanded,
        }
    }
}

impl<'a> ComponentInterface for AstPanel<'a> {
    fn render(&self, _grid: Grid) -> Result<Component<'_>, anyhow::Error> {
        let current_zoomed_node = match self.current_zoomed_node {
            None => self.tree,
            Some(current_zoomed_node) => self.tree.get_path(current_zoomed_node).as_node(),
        };
        Ok(soft! {
            %FlexColumn
              children => [
                %Node::new(current_zoomed_node, 0, true, self.are_locations_expanded)
              ]
              overflow_y => hidden
        })
    }

    fn flex_grow(&self) -> Option<f64> {
        Some(1.0)
    }
}

pub struct Node<'a> {
    pub node: &'a ast::Node,
    pub nesting_level: usize,
    pub should_print_top_and_bottom_lines: bool,
    pub are_locations_expanded: bool,
}

impl<'a> Node<'a> {
    pub fn new(
        node: &'a ast::Node,
        nesting_level: usize,
        should_print_top_and_bottom_lines: bool,
        are_locations_expanded: bool,
    ) -> Self {
        Self {
            node,
            nesting_level,
            should_print_top_and_bottom_lines,
            are_locations_expanded,
        }
    }
}

impl<'a> ComponentInterface for Node<'a> {
    fn render(&self, _grid: Grid) -> Result<Component<'_>, anyhow::Error> {
        Ok(
            match !self.are_locations_expanded && self.node.is_location {
                false => soft! {
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
                                            %NodeChild::new(child, self.nesting_level, self.are_locations_expanded)
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
                },
                true => soft! {
                    %FlexColumn
                        children => [
                            %Text
                              children => [
                                %InitialSpaces::new(self.nesting_level)
                                %Text &self.node.type_
                                %Text " { ... }"
                              ]
                        ]
                },
            },
        )
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
    pub are_locations_expanded: bool,
}

impl<'a> NodeChild<'a> {
    pub fn new(
        node_child: &'a ast::NodeChild,
        nesting_level: usize,
        are_locations_expanded: bool,
    ) -> Self {
        Self {
            node_child,
            nesting_level,
            are_locations_expanded,
        }
    }
}

impl<'a> ComponentInterface for NodeChild<'a> {
    fn render(&self, _grid: Grid) -> Result<Component<'_>, anyhow::Error> {
        Ok(match &self.node_child.value {
            ast::Value::Scalar(value) => soft! {
                %Text children => [
                  %InitialSpaces::new(self.nesting_level + 1)
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
                        %Array::new(list, self.nesting_level, self.are_locations_expanded)
                        %Text children => [
                          %InitialSpaces::new(self.nesting_level + 1)
                          %Text "]"
                        ]

                    ]
                },
            },
            ast::Value::Node(node) => match !self.are_locations_expanded && node.is_location {
                false => soft! {
                    %FlexColumn children => [
                      %Text children => [
                        %InitialSpaces::new(self.nesting_level + 1)
                        %NodeChildName::new(&self.node_child.name)
                        %Text " "
                        %Text &node.type_
                        %Text " {"
                      ]
                      %Node::new(node, self.nesting_level + 1, false, self.are_locations_expanded)
                      %Text children => [
                        %InitialSpaces::new(self.nesting_level + 1)
                        %Text "}"
                      ]
                    ]
                },
                true => soft! {
                    %FlexColumn children => [
                      %Text children => [
                        %InitialSpaces::new(self.nesting_level + 1)
                        %NodeChildName::new(&self.node_child.name)
                        %Text " "
                        %Text &node.type_
                        %Text " { ... }"
                      ]
                    ]
                },
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
    pub are_locations_expanded: bool,
}

impl<'a> Array<'a> {
    pub fn new(nodes: &'a [ast::Node], nesting_level: usize, are_locations_expanded: bool) -> Self {
        assert!(!nodes.is_empty());
        Self {
            nodes,
            nesting_level,
            are_locations_expanded,
        }
    }
}

impl<'a> ComponentInterface for Array<'a> {
    fn render(&self, _grid: Grid) -> Result<Component<'_>, anyhow::Error> {
        Ok(soft! {
            %FlexColumn
              children => self.nodes.into_iter().map(|node| -> Result<_, anyhow::Error> {
                  Ok(soft! {
                      %Node::new(node, self.nesting_level + 2, true, self.are_locations_expanded)
                  })
              }).collect::<Result<Vec<_>, _>>()?
        })
    }
}
