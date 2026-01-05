use std::borrow::Cow;

use oelung::{anyhow, soft, Component, ComponentInterface, Grid};
use ropey::{Rope, RopeSlice};

use crate::{explorer, Location, Node};

pub struct EditorPanel<'a> {
    pub source_text: &'a Rope,
    pub cursor_position: explorer::Position,
    pub current_zoomed_node: Option<&'a Node>,
}

impl<'a> EditorPanel<'a> {
    pub fn new(
        source_text: &'a Rope,
        cursor_position: explorer::Position,
        current_zoomed_node: Option<&'a Node>,
    ) -> Self {
        Self {
            source_text,
            cursor_position,
            current_zoomed_node,
        }
    }

    fn render_line(
        &self,
        line: RopeSlice<'_>,
        line_num: usize,
    ) -> Result<Component<'_>, anyhow::Error> {
        let line_len = explorer::line_len(&line);
        let line: Cow<'_, str> = line.into();
        let line = &line[..line_len];
        let highlighted_range = self
            .current_zoomed_node
            .and_then(|current_zoomed_node| current_zoomed_node.range)
            .filter(|range| range.overlaps_with_line_num(line_num))
            .map(|range| {
                (
                    match range.start {
                        Location::OffsetAndPosition { position, .. } => {
                            if position.line == line_num && position.column > 0 {
                                Some(position.column)
                            } else {
                                None
                            }
                        }
                    },
                    match range.end {
                        Location::OffsetAndPosition { position, .. } => {
                            if position.line == line_num {
                                Some(position.column)
                            } else {
                                None
                            }
                        }
                    },
                )
            });
        Ok(match highlighted_range {
            None => soft! {
                %Text &line
            },
            Some(highlighted_range) => match highlighted_range {
                (None, None) => soft! {
                    %Text
                      text => &line
                      background_color => Ansi(52)
                },
                (Some(start), None) => soft! {
                    %Text children => [
                      %Text &line[..start]
                      %Text
                        text => &line[start..]
                        background_color => Ansi(52)
                    ]
                },
                (None, Some(end)) => soft! {
                    %Text children => [
                      %Text
                        text => &line[..end]
                        background_color => Ansi(52)
                      %Text &line[end..]
                    ]
                },
                (Some(start), Some(end)) => soft! {
                    %Text children => [
                      %Text &line[..start]
                      %Text
                        text => &line[start..end]
                        background_color => Ansi(52)
                      %Text &line[end..]
                    ]
                },
            },
        })
    }
}

impl<'a> ComponentInterface for EditorPanel<'a> {
    fn render(&self, _grid: Grid) -> Result<Component<'_>, anyhow::Error> {
        Ok(soft! {
            %FlexColumn
              children => self
                .source_text
                .lines()
                .enumerate()
                .map(|(line_num, line)| self.render_line(line, line_num))
                .collect::<Result<_, _>>()?
              cursor => %Cursor.Relative
                x => self.cursor_position.column
                y => self.cursor_position.row
        })
    }

    fn flex_grow(&self) -> Option<f64> {
        Some(1.0)
    }
}
