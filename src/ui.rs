use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame){
    f.render_widget(
        Paragraph::new(format!(
            "
            Press 1 to generate new password \n\
            Press q to quit\n\
            Password: {}
            ", app.password
        ))
            .block(
                Block::default()
                    .title("Password App")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center),
        f.size(),
    )
}