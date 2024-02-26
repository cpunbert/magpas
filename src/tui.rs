use std::{io, panic};
use std::io::{Result};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};


pub type CrosstermTerminal =
    ratatui::Terminal<ratatui::backend::CrosstermBackend<io::Stderr>>;

use crate::{app::App, ui,};




pub struct Tui{
    terminal: CrosstermTerminal,
    pub events: crate::event::EventHandler,
}

impl Tui{
    pub fn new(terminal: CrosstermTerminal, events: crate::event::EventHandler) -> Self{
        Self {terminal, events}
    }

    pub fn enter(&mut self) -> Result<()>{
        terminal::enable_raw_mode()?;
        crossterm::execute!(
            io::stderr(),
            EnterAlternateScreen,
            EnableMouseCapture
        )?;


        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal");
            panic_hook(panic);

        }));
        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())

    }


    fn reset() -> Result<()>{
        terminal::disable_raw_mode()?;
        crossterm::execute!(
            io::stderr(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()>{
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }

    pub fn draw(&mut self, app: &mut App) -> Result<()>{
        self.terminal.draw(|frame| ui::render(app, frame))?;
        Ok(())
    }


}
