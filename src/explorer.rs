use std::borrow::Cow;

use oelung::{anyhow, soft, Component, ComponentInterface, Grid};
use ropey::Rope;

use crate::{syn::Parser, AstPanel, EditorPanel, Error, Node, Parse};

#[derive(Debug)]
pub struct AstExplorer {
    pub tree: Node,
    pub source_text: Rope,
}

impl AstExplorer {
    pub fn try_new(text: Rope) -> Result<Self, Error> {
        let parser = Parser::new();
        let tree = parser.parse(&Cow::<'_, str>::from(&text))?;
        Ok(Self {
            tree,
            source_text: text,
        })
    }
}

impl<'a> ComponentInterface for &'a AstExplorer {
    fn render(&self, _grid: Grid) -> Result<Component<'_>, anyhow::Error> {
        Ok(soft! {
            %FlexRow children => [
              %EditorPanel::new(&self.source_text)
              %AstPanel::new(&self.tree)
            ]
        })
    }

    fn flex_grow(&self) -> Option<f64> {
        Some(1.0)
    }
}
