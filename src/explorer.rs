use std::borrow::Cow;
use std::pin::Pin;

use oelung::{anyhow, soft, Component, ComponentInterface, Grid};
use oelung_lantern::ReceiveEvent;
use ropey::{Rope, RopeSlice};
use squalid::_d;

use crate::{
    ast, node_path_parent_node, syn::Parser, AstPanel, EditorPanel, Error, Node, NodePath, Parse,
};

#[derive(Debug)]
pub struct AstExplorer {
    pub tree: Node,
    pub source_text: Rope,
    pub cursor_position: Position,
    pub sticky_cursor_position_column: Option<u16>,
    pub current_zoomed_node: Option<NodePath>,
    pub are_locations_expanded: bool,
}

impl AstExplorer {
    pub fn try_new(text: Rope) -> Result<Self, Error> {
        let parser = Parser::new();
        let tree = parser.parse(&Cow::<'_, str>::from(&text))?;
        Ok(Self {
            tree,
            source_text: text,
            cursor_position: Position { row: 0, column: 0 },
            sticky_cursor_position_column: _d(),
            current_zoomed_node: _d(),
            are_locations_expanded: true,
        })
    }

    fn max_allowed_column(&self) -> u16 {
        match line_len(&self.source_text.line(usize::from(self.cursor_position.row))) {
            0 => 0,
            line_len => u16::try_from(line_len).unwrap() - 1,
        }
    }

    fn remember_sticky_cursor_position_column(&mut self) {
        self.sticky_cursor_position_column = Some(self.cursor_position.column);
    }
}

impl<'a> ComponentInterface for &'a AstExplorer {
    fn render(&self, _grid: Grid) -> Result<Component<'_>, anyhow::Error> {
        Ok(soft! {
            %FlexRow
              children => [
                %EditorPanel::new(&self.source_text, self.cursor_position)
                %AstPanel::new(&self.tree, self.current_zoomed_node.as_ref(), self.are_locations_expanded)
              ]
              flex_grow => 1
        })
    }

    fn flex_grow(&self) -> Option<f64> {
        Some(1.0)
    }
}

impl ReceiveEvent<Event> for AstExplorer {
    fn receive<TQueueEffect: FnMut(Pin<Box<dyn Future<Output = ()> + Send + 'static>>)>(
        &mut self,
        event: &Event,
        _queue_effect: TQueueEffect,
    ) {
        match event {
            Event::CursorMovement(CursorMovement::Up) => {
                if self.cursor_position.row > 0 {
                    self.cursor_position.row -= 1;
                    if let Some(sticky_cursor_position_column) = self.sticky_cursor_position_column
                    {
                        self.cursor_position.column = sticky_cursor_position_column;
                    }
                    if self.cursor_position.column > self.max_allowed_column() {
                        self.cursor_position.column = self.max_allowed_column();
                    }
                }
            }
            Event::CursorMovement(CursorMovement::Down) => {
                if usize::from(self.cursor_position.row) < self.source_text.len_lines() - 1 {
                    self.cursor_position.row += 1;
                    if let Some(sticky_cursor_position_column) = self.sticky_cursor_position_column
                    {
                        self.cursor_position.column = sticky_cursor_position_column;
                    }
                    if self.cursor_position.column > self.max_allowed_column() {
                        self.cursor_position.column = self.max_allowed_column();
                    }
                }
            }
            Event::CursorMovement(CursorMovement::Left) => {
                if self.cursor_position.column > 0 {
                    self.cursor_position.column -= 1;
                    self.remember_sticky_cursor_position_column();
                }
            }
            Event::CursorMovement(CursorMovement::Right) => {
                if self.cursor_position.column < self.max_allowed_column() {
                    self.cursor_position.column += 1;
                    self.remember_sticky_cursor_position_column();
                }
            }
            Event::ZoomAst => {
                self.current_zoomed_node = self.tree.get_path_of_smallest_containing_node(
                    ast::Position {
                        line: usize::from(self.cursor_position.row),
                        column: usize::from(self.cursor_position.column),
                    },
                    _d(),
                );
            }
            Event::ToggleLocations => {
                self.are_locations_expanded = !self.are_locations_expanded;
            }
            Event::PopZoomedAst => {
                if let Some(current_zoomed_node) = self.current_zoomed_node.as_ref() {
                    self.current_zoomed_node = node_path_parent_node(current_zoomed_node);
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

pub enum Event {
    CursorMovement(CursorMovement),
    ZoomAst,
    ToggleLocations,
    PopZoomedAst,
}

pub fn line_len(line: &RopeSlice<'_>) -> usize {
    let line_len = line.len_bytes();
    match line.byte(line_len - 1) {
        // \n
        0x0A => line_len - 1,
        _ => line_len,
    }
}
