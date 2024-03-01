use crossterm::event::{EnableMouseCapture, DisableMouseCapture, Event, KeyCode, KeyEventKind};
use crossterm::{event, execute};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, LeaveAlternateScreen, EnterAlternateScreen};
use std::io;
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::Terminal;
use std::{error::Error,};
use crate::app::{App, CurrentlyEditing, CurrentScreen};
use crate::ui::ui;

mod app;

mod ui;
mod password;
fn main() ->Result<(), Box<dyn Error>>{
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);



    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res{
        if do_print{
            app.print_json()?;
        }
    } else if let Err(err) = res{
        println!("{err:?}");
    }
    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<bool>{
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()?{
            if key.kind == KeyEventKind::Release{
                continue;
            }
            match app.current_screen{

                CurrentScreen::Main => match key.code{
                    KeyCode::Char('e') =>{
                        app.current_screen = CurrentScreen::Editing;
                        app.currently_editing = Some(CurrentlyEditing::Login);
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    _ => {}
                },

                CurrentScreen::Exiting => match key.code{
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') =>{
                        return Ok(false);
                    }
                    _ => {}
                },
                CurrentScreen::Editing if key.kind == KeyEventKind::Press =>{
                    match key.code{
                        KeyCode::Enter =>{
                            if let Some(editing) = &app.currently_editing{ //the last field should be pass str if so -> generate password and then save
                                match editing{
                                    CurrentlyEditing::Login =>{
                                        app.currently_editing = Some(CurrentlyEditing::Password);
                                    }
                                    CurrentlyEditing::Password =>{
                                        app.save_key_value();
                                        app.current_screen = CurrentScreen::Main;
                                    }
                                }
                            }
                        }
                        KeyCode::Backspace =>{
                            if let Some(editing) = &app.currently_editing{
                                match editing{
                                    CurrentlyEditing::Login =>{
                                        app.login_input.pop();
                                    }
                                    CurrentlyEditing::Password => {
                                        app.password_input.pop();
                                    }
                                }
                            }
                        }
                        KeyCode::Esc =>{
                            app.current_screen = CurrentScreen::Main;
                            app.currently_editing = None;
                        }
                        KeyCode::Tab =>{
                            app.toggle_editing();
                        }
                        KeyCode::Char(value) => {
                            if let Some(editing) = &app.currently_editing{
                                match editing{
                                    CurrentlyEditing::Login =>{
                                        app.login_input.push(value);
                                    }
                                    CurrentlyEditing::Password =>{
                                        app.password_input.push(value);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }





        }




    }
}