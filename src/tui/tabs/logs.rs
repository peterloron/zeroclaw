//! Logs tab -- placeholder for PR-15.

use ratatui::prelude::*;
use ratatui::widgets::*;

pub fn render(frame: &mut Frame, area: Rect) {
    let placeholder = Paragraph::new("Logs (not yet implemented)")
        .block(Block::default().borders(Borders::ALL).title(" Logs "));
    frame.render_widget(placeholder, area);
}
