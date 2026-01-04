use std::borrow::Cow;
use std::pin::Pin;

use oelung::{anyhow, soft, Component, ComponentInterface, Grid};
use oelung_lantern::ReceiveEvent;
use ropey::Rope;

use crate::{syn::Parser, AstPanel, EditorPanel, Error, Node, Parse};

#[derive(Debug)]
pub struct AstExplorer {
    pub tree: Node,
    pub source_text: Rope,
    pub cursor_position: Position,
}

impl AstExplorer {
    pub fn try_new(text: Rope) -> Result<Self, Error> {
        let parser = Parser::new();
        let tree = parser.parse(&Cow::<'_, str>::from(&text))?;
        Ok(Self {
            tree,
            source_text: text,
            cursor_position: Position { row: 0, column: 0 },
        })
    }
}

impl<'a> ComponentInterface for &'a AstExplorer {
    fn render(&self, _grid: Grid) -> Result<Component<'_>, anyhow::Error> {
        Ok(soft! {
            %FlexRow
              children => [
                %EditorPanel::new(&self.source_text, self.cursor_position)
                %AstPanel::new(&self.tree)
              ]
              flex_grow => 1
        })
    }

    fn flex_grow(&self) -> Option<f64> {
        Some(1.0)
    }
}

impl ReceiveEvent<CursorMovement> for AstExplorer {
    fn receive<TQueueEffect: FnMut(Pin<Box<dyn Future<Output = ()> + Send + 'static>>)>(
        &mut self,
        event: &CursorMovement,
        _queue_effect: TQueueEffect,
    ) {
        match event {
            CursorMovement::Up => {
                if self.cursor_position.row > 0 {
                    self.cursor_position.row -= 1;
                }
            }
            CursorMovement::Down => {
                if usize::from(self.cursor_position.row) < self.source_text.len_lines() - 1 {
                    self.cursor_position.row += 1;
                }
            }
            CursorMovement::Left => {
                if self.cursor_position.column > 0 {
                    self.cursor_position.column -= 1;
                }
            }
            CursorMovement::Right => {
                if usize::from(self.cursor_position.column)
                    < self
                        .source_text
                        .line(usize::from(self.cursor_position.row))
                        .len_bytes()
                        - 1
                {
                    self.cursor_position.column += 1;
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub row: u16,
    pub column: u16,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CursorMovement {
    Up,
    Down,
    Left,
    Right,
}
