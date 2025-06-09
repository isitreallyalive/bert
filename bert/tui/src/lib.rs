use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    prelude::*,
};
use std::{io, time::Duration};

const TICK_RATE: Duration = Duration::from_millis(100);
type Terminal = ratatui::Terminal<CrosstermBackend<io::Stdout>>;

#[derive(Default)]
struct App {
    exit: bool,
}

impl App {
    fn run(&mut self, terminal: &mut Terminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(Line::from("hi"), frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(TICK_RATE)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => self.exit = true,
                    _ => {}
                }
            }
        }

        Ok(())
    }
}

/// Run the TUI application
pub fn run() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
