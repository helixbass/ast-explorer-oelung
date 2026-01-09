use std::pin::Pin;

use crossterm::event::{Event, EventStream, KeyCode};
use indoc::indoc;
use oelung::{soft, BackendInterface, Renderer, RendererBuilder};
use oelung_lantern::{generate_sender, is_ctrl_char_press, mpsc::Sender, ReceiveEvent};
use tokio::sync::mpsc::channel;
use tokio_stream::StreamExt;
use washtank::editor;

use ast_explorer_oelung::{explorer, syn::Parser, AstExplorer};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut renderer = RendererBuilder::default().build()?;

    let (sender, mut receiver) = channel::<World>(100);

    listen_to_crossterm_events(CrosstermSender::from(sender.clone()));

    let text = strip_trailing_newline(indoc!(
        r#"
            const TIPS: &[&str] = &[
                "Click on any AST node with a '+' to expand it",

                "Hovering over a node highlights the \
                corresponding location in the source code",

                "Shift click on an AST node to expand the whole subtree",
            ];

            pub fn print_tips() {
                for (i, tip) in TIPS.iter().enumerate() {
                    println!("Tip {}: {}.", i, tip);
                }
            }
        "#
    ));
    let mut explorer = AstExplorer::try_new(
        text.to_owned(),
        Box::new(EditorSender::from(sender.clone())),
        renderer.backend.size()?,
        Box::new(Parser::new()),
    )
    .await?;

    render_screen(&mut renderer, &explorer)?;

    while let Some(world) = receiver.recv().await {
        let mut queued_effects: Vec<Pin<Box<dyn Future<Output = ()> + Send + 'static>>> = vec![];
        match world {
            World::Crossterm(Event::Key(key)) if key.code == KeyCode::Char('q') => {
                break;
            }
            World::Crossterm(Event::Key(key)) if key.code == KeyCode::Enter => {
                explorer.receive(&explorer::Event::ZoomAst, |future| {
                    queued_effects.push(future)
                })?;
                render_screen(&mut renderer, &explorer)?;
            }
            World::Crossterm(event) if is_ctrl_char_press(&event, 's') => {
                explorer.receive(&explorer::Event::ToggleLocations, |future| {
                    queued_effects.push(future)
                })?;
                render_screen(&mut renderer, &explorer)?;
            }
            World::Crossterm(event) if is_ctrl_char_press(&event, 'u') => {
                explorer.receive(&explorer::Event::PopZoomedAst, |future| {
                    queued_effects.push(future)
                })?;
                render_screen(&mut renderer, &explorer)?;
            }
            World::Crossterm(event) => {
                explorer.receive(&explorer::Event::Crossterm(event), |future| {
                    queued_effects.push(future)
                })?;
                render_screen(&mut renderer, &explorer)?;
            }
            World::Editor(editor::Happened::Quit) => unreachable!(),
        }
        for effect in queued_effects {
            tokio::spawn(effect);
        }
    }

    Ok(())
}

fn render_screen(renderer: &mut Renderer, explorer: &AstExplorer) -> Result<(), anyhow::Error> {
    renderer.render(soft! {
      %explorer
    })?;

    Ok(())
}

enum World {
    Crossterm(Event),
    Editor(editor::Happened),
}

generate_sender!(World, Crossterm, Event);
generate_sender!(World, Editor, editor::Happened);

fn listen_to_crossterm_events(sender: CrosstermSender) {
    tokio::spawn(async move {
        let mut event_stream = EventStream::new();

        while let Some(Ok(event)) = event_stream.next().await {
            sender.send(event).await;
        }

        panic!("kill everything")
    });
}

fn strip_trailing_newline(file_contents: &str) -> &str {
    if file_contents.ends_with("\n") {
        &file_contents[..file_contents.len() - 1]
    } else {
        file_contents
    }
}
