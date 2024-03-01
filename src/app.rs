use std::collections::HashMap;
use anyhow::Result;
pub enum CurrentScreen{
    Main,
    Editing,
    Exiting,
}


pub enum CurrentlyEditing{
    Login,
    Password,
}

pub struct App{
    pub login_input: String,
    pub password_input: String,
    pub pairs: HashMap<String,String>,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
}

impl App{
    pub fn new() -> App{
        App{
            login_input: String::new(),
            password_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }

    pub fn save_key_value(&mut self){
        self.pairs
            .insert(self.login_input.clone(), self.password_input.clone());
        self.login_input = String::new();
        self.password_input = String::new();
        self.currently_editing = None;
    }

    pub fn toggle_editing(&mut self){
        if let Some(edit_mode) = &self.currently_editing{
            match edit_mode {
                CurrentlyEditing::Login => {
                    self.currently_editing = Some(CurrentlyEditing::Password)
                }
                CurrentlyEditing::Password => {
                    self.currently_editing = Some(CurrentlyEditing::Login)
                }
            };
        }else {
            self.currently_editing = Some(CurrentlyEditing::Login)
        }
    }

    pub fn print_json(&self) -> Result<()>{
        let output = serde_json::to_string(&self.pairs)?;
        println!("{}", output);
        Ok(())
    }



}
