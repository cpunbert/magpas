use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap};

use crate::app::{App, CurrentlyEditing, CurrentScreen};

pub fn ui(f: &mut Frame, app:&App){

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Create new Password",
        Style::default().fg(Color::Green),
    ))
        .block(title_block);

    f.render_widget(title,chunks[0]);
    let mut list_items = Vec::<ListItem>::new();

    for key in app.pairs.keys(){
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{: <25} : {}",key, app.pairs.get(key).unwrap()),
            Style::default().fg(Color::Yellow),
        ))))
    }

    let current_navigation_text= vec![
        match app.current_screen{
            CurrentScreen::Main =>{
                Span::styled("Normal Mode", Style::default().fg(Color::Green))
            }
            CurrentScreen::Editing =>{
                Span::styled("Editing Mode", Style::default().fg(Color::Yellow))
            }
            CurrentScreen::Exiting =>{
                Span::styled("Exiting", Style::default().fg(Color::LightRed))
            }

        }.to_owned(),
        Span::styled(" | ", Style::default().fg(Color::White)),
        if let Some(editing) = &app.currently_editing{
            match editing {
                CurrentlyEditing::Login => Span::styled(
                    "Editing Login",
                    Style::default().fg(Color::Green),
                ),
                CurrentlyEditing::Password => Span::styled(
                    "Editing Password",
                    Style::default().fg(Color::LightGreen),
                ),
            }
        } else {
            Span::styled(
                "Not editing anything",
                Style::default().fg(Color::DarkGray),
            )
        }
    ];
    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "(q) to quit / (e) to make new pair",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Editing => Span::styled(
                "(ESC) to cancel/(Tab) to switch boxes/enter to complete",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exiting => Span::styled(
                "(q) to quit / (e) to make new pair",
                Style::default().fg(Color::Red),
            ),
        }
    };

    let key_notes_footer = Paragraph::new(Line::from(current_keys_hint))
        .block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);


    let list = List::new(list_items);

    f.render_widget(mode_footer, footer_chunks[0]);
    f.render_widget(list, chunks[1]);
    f.render_widget(key_notes_footer, footer_chunks[1]);


    if let Some(editing) = &app.currently_editing{
        let popup_block = Block::default()
            .title("Enter a new Login and Password")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));
        let area = centered_rect(60,25, f.size());
        f.render_widget(popup_block, area);

        let popup_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50,)
            ])
            .split(area);


        let mut login_block = Block::default().title("Login").borders(Borders::ALL);
        let mut password_block = Block::default().title("Password").borders(Borders::ALL);
        //add more blocks for pass str and len


        let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

        match editing{
            CurrentlyEditing::Login => login_block = login_block.style(active_style),
            CurrentlyEditing::Password => password_block = password_block.style(active_style)
        };
        let login_text = Paragraph::new(app.login_input.clone()).block(login_block);
        let password_text = Paragraph::new(app.password_input.clone()).block(password_block);

        f.render_widget(login_text, popup_chunks[0]);
        f.render_widget(password_text, popup_chunks[1])
    }

    if let CurrentScreen::Exiting = app.current_screen{
        f.render_widget(Clear, f.size());
        let popup_block = Block::default()
            .title("Y/N")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let exit_text = Text::styled(
            "Would you like to save login and password?(y/n)",
            Style::default().fg(Color::Red),
        );
        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap{trim:false});

        let area = centered_rect(60, 25, f.size());
        f.render_widget(exit_paragraph, area)
    }






    fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect{
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) /2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) /2)
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x)/2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x)/2),
            ])
            .split(popup_layout[1])[1]

    }



}