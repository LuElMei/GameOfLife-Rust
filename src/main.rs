use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, layout::Rect, style::Color, symbols, widgets::{Block, Paragraph, Widget, canvas::{Canvas, Line, Map, MapResolution, Rectangle}}};


enum MyEvent {
    AppQuit,
    PrintPrompt,
}

enum MyGameState {
    Menu,
    Simulation,
    Pause,
}

struct MyApp {
    game_state: MyGameState,
    living_cells: i32,
    dead_cells: i32,
}

struct Cell {
    x_coor: f64,
    y_coor: f64,
    is_living: bool,
}

impl MyApp {
    fn new() -> Self {
        Self {
            game_state: MyGameState::Menu,
            living_cells: 0,
            dead_cells: 0,
        }
    }
}
fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let app = MyApp::new();
    let result = run_app(&mut terminal);
    ratatui::restore();
    result
}

fn run_app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.clear()?;
        terminal.draw(|frame| frame.render_widget(draw_grid(120,60), frame.area().resize(size)))?;
        if let Some(my_event) = handle_events()? {
            match my_event {
                MyEvent::AppQuit => std::process::exit(0),
                _ => {}
            }
        }
    }
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

fn draw_grid(width: i32, height: i32) -> impl Widget {
    Canvas::default()
    .marker(symbols::Marker::Braille)
    .block(Block::bordered().title("Canvas"))
    .x_bounds([0.0, 80.0])
    .y_bounds([0.0, 40.0])
    .paint(move |ctx| {
        for y in (0..=height).step_by(2) {
        ctx.draw(&Line::new(0.0, y as f64, 120.0, y as f64, Color::DarkGray));
        }

        for x in (0..=width).step_by(5) {
            ctx.draw(&Line::new(x as f64, 0.0, x as f64, 120.0, Color::DarkGray));
        }
    })
}
