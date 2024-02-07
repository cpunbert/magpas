use std::env;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand, execute};
use ratatui::{Frame, prelude::{CrosstermBackend, Stylize, Terminal}, widgets::Paragraph};
use std::io::{stdout, Result};
use rand::Rng;


enum PasswordStrength{
    LowerCase, //lower case letters
    UpperCase, // lower case + upper case letters
    Numbers, // upper + lower case  + numbers
    Symbols //upper + lower + numbers + symbols
}
struct Password{
    password: String,
    exit: bool,
}

fn main()-> Result<()>{
    startup()?;
    let status = run();
    shutdown()?;
    status?;
    Ok(())
}

fn run() -> Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut password= Password{password: generate_password(24, PasswordStrength::Symbols), exit: false};

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

fn ui(password: &Password, frame: &mut Frame){
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


fn generate_password(pass_len: i32,pass_strength: PasswordStrength) -> String{

    let mut rng = rand::thread_rng();
    let password_set:&[u8] = match pass_strength {
        PasswordStrength::LowerCase =>
            b"abcdefghijklmnopqrstuwxyz"
        ,
        PasswordStrength::UpperCase =>
            b"abcdefghijklmnopqrstuwvxyz\
            ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        ,
        PasswordStrength::Numbers =>
            b"abcdefghijklmnopqrstuwvxyz\
            ABCDEFGHIJKLMNOPQRSTUVWXYZ\
            0123456789"
        ,
        PasswordStrength::Symbols =>
            b"bcdefghijklmnopqrstuwvxyz\
            ABCDEFGHIJKLMNOPQRSTUVWXYZ\
            0123456789!@#$%^&*()_+-={}[]|:;<>,.?/"


    };

    let password: String = (0..pass_len)
        .map(|_|{
            let n = rng.gen_range(0..password_set.len());
            password_set[n] as char
        })
        .collect();
    return password



}

