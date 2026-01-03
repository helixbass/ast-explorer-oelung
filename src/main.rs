use crossterm::event::{Event, EventStream, KeyCode};
use indoc::indoc;
use oelung::{soft, Renderer};
use oelung_lantern::{generate_sender, mpsc::Sender};
use tokio::sync::mpsc::channel;
use tokio_stream::StreamExt;

use ast_explorer_oelung::{syn::Parser, AstExplorer, Parse};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut renderer = Renderer::try_new()?;

    let (sender, mut receiver) = channel::<World>(100);

    listen_to_crossterm_events(CrosstermSender::from(sender.clone()));

    let text = indoc!(
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
    );
    let parser = Parser::new();
    let explorer = AstExplorer::new(parser.parse(text)?);

    render_screen(&mut renderer, &explorer)?;

    while let Some(world) = receiver.recv().await {
        match world {
            World::Crossterm(Event::Key(key)) if key.code == KeyCode::Char('q') => {
                break;
            }
            _ => {}
        }
    }

    Ok(())
}

fn render_screen(renderer: &mut Renderer, explorer: &AstExplorer) -> Result<(), anyhow::Error> {
    renderer.render(soft! {
      %FlexColumn
        children => [
          %explorer
          %Text " (hit q to quit)"
        ]
    })?;

    Ok(())
}

enum World {
    Crossterm(Event),
}

generate_sender!(World, Crossterm, Event);

fn listen_to_crossterm_events(sender: CrosstermSender) {
    tokio::spawn(async move {
        let mut event_stream = EventStream::new();

        while let Some(Ok(event)) = event_stream.next().await {
            sender.send(event).await;
        }

        panic!("kill everything")
    });
}
