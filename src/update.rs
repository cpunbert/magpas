use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;
use crate::app::PasswordStrength::Symbols;


pub fn update(app: &mut App, key_event: KeyEvent){
    match key_event.code {
        KeyCode::Esc |KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') |KeyCode::Char('C') =>{
            if key_event.modifiers == KeyModifiers::CONTROL{
                app.quit()
            }
        }
        KeyCode::Char('g') | KeyCode::Enter => app.gen_pass(20, Symbols),
        _ => {}
    };
}