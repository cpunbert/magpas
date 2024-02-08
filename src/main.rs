use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand, execute};
use ratatui::{Frame, prelude::{CrosstermBackend, Stylize, Terminal}, widgets::Paragraph};
use std::io::{stdout, Result};
use crate::app::App;


pub mod app;
pub mod event;
pub mod ui;
pub mod tui;
pub mod update;






fn main()-> Result<()>{
    startup()?;
    let status = run();
    shutdown()?;
    status?;
    Ok(())
}

fn run() -> Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut password= App{password: generate_password(24, PasswordStrength::Symbols), exit: false};

    loop{
        terminal.draw(|frame|{
            ui(&password, frame);

        })?;
        update(&mut password)?;

        if password.exit{
            break
        }

    }
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())

}
fn startup() -> Result<()>{
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Ok(())
}
fn shutdown() -> Result<()>{
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn ui(password: &App, frame: &mut Frame){
    frame.render_widget(
        Paragraph::new(format!("password: {}", password.password)).green().on_light_magenta(),
        frame.size(),
    );
}





fn update(password: &mut Password) ->Result<()>{
    if event::poll(std::time::Duration::from_millis(16))?{
        if let event::Event::Key(key) = event::read()?{
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('1') => password.password = generate_password(24, PasswordStrength::Symbols),
                    KeyCode::Char('q') => password.exit = true,
                    _ => {},
                }
            }
        }
    }
    Ok(())
}




