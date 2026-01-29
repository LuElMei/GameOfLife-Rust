use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, widgets::Paragraph};


enum MyEvent {
    AppQuit,
    PrintPrompt,
}
fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run_app(&mut terminal);
    ratatui::restore();
    result
}

fn run_app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| render(frame))?;
        if let Some(my_event) = handle_events()? {
            match my_event {
                MyEvent::AppQuit => std::process::exit(0),
                _ => {}
            }
        }
    }
}

fn render(frame: &mut Frame) {
    let text = Paragraph::new("Hello World!");
    frame.render_widget(text, frame.area());
}

fn handle_events() -> std::io::Result<Option<MyEvent>> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => {
            match key.code {
                KeyCode::Char('q') => return Ok(Some(MyEvent::AppQuit)),
                _ => {}
            }
        }
        _ => {}
    }
    Ok(None)
}
