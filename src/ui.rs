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
            Press g or enter to generate new password \n\
            Press q, Esc or Ctrl-C to quit\n\
            Password: {}
            ", app.password
        ))
            .block(
                Block::default()
                    .title("MagPas")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center),
        f.size(),
    )
}