pub use tui_logger::{LevelFilter, TuiTracingSubscriberLayer, init_logger};
use tui_logger::{TuiLoggerLevelOutput, TuiLoggerWidget};

use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    prelude::*,
};
use std::{io, time::Duration};

const TICK_RATE: Duration = Duration::from_millis(100);
type Terminal = ratatui::Terminal<CrosstermBackend<io::Stdout>>;

#[derive(Debug, thiserror::Error)]
pub enum TuiError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Logger(#[from] tui_logger::TuiLoggerError),
}

#[derive(Default)]
struct App {
    exit: bool,
}

impl App {
    fn run(&mut self, terminal: &mut Terminal) -> Result<(), TuiError> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        TuiLoggerWidget::default()
            .style_error(Style::default().fg(Color::Red))
            .style_debug(Style::default().fg(Color::Green))
            .style_warn(Style::default().fg(Color::Yellow))
            .style_trace(Style::default().fg(Color::Magenta))
            .style_info(Style::default().fg(Color::Cyan))
            .output_separator(':')
            .output_timestamp(Some("%H:%M:%S".to_string()))
            .output_level(Some(TuiLoggerLevelOutput::Abbreviated))
            .output_target(true)
            .output_file(true)
            .output_line(true)
            .render(frame.area(), frame.buffer_mut());
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
pub fn run() -> Result<(), TuiError> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
