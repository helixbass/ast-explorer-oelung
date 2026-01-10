use std::borrow::Cow;
use std::fmt;
use std::pin::Pin;

use crossterm::event;
use oelung::{anyhow, soft, Component, ComponentInterface, Grid, Size};
use oelung_lantern::{mpsc::Sender, ReceiveEvent};
use ropey::RopeSlice;
use smol_str::ToSmolStr;
use squalid::_d;
use washtank::{editor, ConfigBuilder, Editor, EventAggregator, InitialFile};

use crate::{ast, node_path_parent_node, AstPanel, Error, Location, Node, NodePath, Parse};

pub struct AstExplorer {
    pub tree: Result<Node, Error>,
    pub editor: Editor,
    pub current_zoomed_node: Option<NodePath>,
    pub are_locations_expanded: bool,
    pub editor_aggregator: EventAggregator,
    pub parser: Box<dyn Parse>,
}

impl AstExplorer {
    pub async fn try_new(
        text: String,
        editor_sender: Box<dyn Sender<editor::Happened>>,
        size: Size,
        mut parser: Box<dyn Parse>,
    ) -> Result<Self, Error> {
        let tree = parser.parse(&text);
        let config = ConfigBuilder::default()
            .initial_file(InitialFile::Anonymous(text))
            .flex_grow(1.0)
            .disallow_folding(true)
            .disallow_ex_command_mode(true)
            .build()
            .unwrap();
        Ok(Self {
            tree,
            parser,
            editor: Editor::try_new(&config, editor_sender, size)
                .await
                .map_err(|err| Error::Washtank(err.to_smolstr()))?,
            current_zoomed_node: _d(),
            are_locations_expanded: true,
            editor_aggregator: EventAggregator::new(&config),
        })
    }

    pub fn re_parse(&mut self) {
        self.tree = self
            .parser
            .parse(&Cow::<'_, str>::from(self.editor.current_file.rope()));
    }

    pub fn tree(&self) -> &Node {
        self.tree.as_ref().unwrap()
    }

    fn tell_editor_to_highlight_zoomed_node<
        TQueueEffect: FnMut(Pin<Box<dyn Future<Output = ()> + Send + 'static>>),
    >(
        &mut self,
        mut queue_effect: TQueueEffect,
    ) -> Result<(), anyhow::Error> {
        let Some(zoomed_node_range) =
            self.current_zoomed_node
                .as_ref()
                .and_then(|current_zoomed_node| {
                    self.tree().get_path(current_zoomed_node).as_node().range
                })
        else {
            self.editor
                .receive(&editor::Event::UnhighlightRange, |future| {
                    queue_effect(future)
                })?;
            return Ok(());
        };

        self.editor.receive(
            &editor::Event::HighlightRange(editor::Range {
                start: match zoomed_node_range.start {
                    Location::OffsetAndPosition { offset, .. } => offset,
                },
                end: match zoomed_node_range.end {
                    Location::OffsetAndPosition { offset, .. } => offset,
                },
            }),
            |future| queue_effect(future),
        )?;

        Ok(())
    }
}

impl<'a> ComponentInterface for &'a AstExplorer {
    fn render(&self, _grid: Grid) -> Result<Component<'_>, anyhow::Error> {
        Ok(soft! {
            %FlexRow
              children => [
                %&self.editor
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
        mut queue_effect: TQueueEffect,
    ) -> Result<(), anyhow::Error> {
        match event {
            Event::ZoomAst => {
                self.current_zoomed_node = self.tree().get_path_of_smallest_containing_node(
                    ast::Position {
                        line: usize::from(self.editor.cursor_position.row),
                        column: usize::from(self.editor.cursor_position.column),
                    },
                    _d(),
                );
                self.tell_editor_to_highlight_zoomed_node(|future| queue_effect(future))?;
            }
            Event::ToggleLocations => {
                self.are_locations_expanded = !self.are_locations_expanded;
            }
            Event::PopZoomedAst => {
                if let Some(current_zoomed_node) = self.current_zoomed_node.as_ref() {
                    self.current_zoomed_node = node_path_parent_node(current_zoomed_node);
                }
                self.tell_editor_to_highlight_zoomed_node(|future| queue_effect(future))?;
            }
            Event::Crossterm(event) => {
                let editor_event = self
                    .editor_aggregator
                    .receive(event, |future| queue_effect(future))?;
                if let Some(editor_event) = editor_event {
                    self.editor
                        .receive(&editor_event, |future| queue_effect(future))?;
                    if editor_event.is_file_contents_mutating() {
                        self.re_parse();
                    }
                }
            }
        }

        Ok(())
    }
}

impl fmt::Debug for AstExplorer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AstExplorer")
            .field("tree", &self.tree)
            // .field("editor", &self.editor)
            .field("current_zoomed_node", &self.current_zoomed_node)
            .field("are_locations_expanded", &self.are_locations_expanded)
            .finish()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub row: u16,
    pub column: u16,
}

pub enum Event {
    ZoomAst,
    ToggleLocations,
    PopZoomedAst,
    Crossterm(event::Event),
}

pub fn line_len(line: &RopeSlice<'_>) -> usize {
    let line_len = line.len_bytes();
    match line.byte(line_len - 1) {
        // \n
        0x0A => line_len - 1,
        _ => line_len,
    }
}
