use oelung::{anyhow, soft, Component, ComponentInterface, Grid};

use crate::{AstPanel, Node};

#[derive(Debug)]
pub struct AstExplorer {
    pub tree: Node,
}

impl AstExplorer {
    pub fn new(tree: Node) -> Self {
        Self { tree }
    }
}

impl<'a> ComponentInterface for &'a AstExplorer {
    fn render(&self, _grid: Grid) -> Result<Component<'_>, anyhow::Error> {
        Ok(soft! {
            %AstPanel::new(&self.tree)
        })
    }

    fn flex_grow(&self) -> Option<f64> {
        Some(1.0)
    }
}
