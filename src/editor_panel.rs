use std::borrow::Cow;

use oelung::{anyhow, soft, Component, ComponentInterface, Grid};
use ropey::Rope;

use crate::explorer;

pub struct EditorPanel<'a> {
    pub source_text: &'a Rope,
    pub cursor_position: explorer::Position,
}

impl<'a> EditorPanel<'a> {
    pub fn new(source_text: &'a Rope, cursor_position: explorer::Position) -> Self {
        Self {
            source_text,
            cursor_position,
        }
    }
}

impl<'a> ComponentInterface for EditorPanel<'a> {
    fn render(&self, _grid: Grid) -> Result<Component<'_>, anyhow::Error> {
        Ok(soft! {
            %FlexColumn
              children => self.source_text.lines().map(|line| -> Result<_, anyhow::Error> {
                Ok(soft! {
                    %Text &Cow::<'_, str>::from(line)[..explorer::line_len(&line)]
                })
              }).collect::<Result<_, _>>()?
              cursor => %Cursor.Relative
                x => self.cursor_position.column
                y => self.cursor_position.row
        })
    }

    fn flex_grow(&self) -> Option<f64> {
        Some(1.0)
    }
}
