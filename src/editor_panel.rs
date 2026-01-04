use std::borrow::Cow;

use oelung::{anyhow, soft, Component, ComponentInterface, Grid};
use ropey::Rope;

pub struct EditorPanel<'a> {
    pub source_text: &'a Rope,
}

impl<'a> EditorPanel<'a> {
    pub fn new(source_text: &'a Rope) -> Self {
        Self { source_text }
    }
}

impl<'a> ComponentInterface for EditorPanel<'a> {
    fn render(&self, _grid: Grid) -> Result<Component<'_>, anyhow::Error> {
        Ok(soft! {
            %FlexColumn
              children => self.source_text.lines().map(|line| -> Result<_, anyhow::Error> {
                Ok(soft! {
                    %Text &Cow::<'_, str>::from(line)
                })
              }).collect::<Result<_, _>>()?
        })
    }

    fn flex_grow(&self) -> Option<f64> {
        Some(1.0)
    }
}
