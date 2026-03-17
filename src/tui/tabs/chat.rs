//! Chat tab -- placeholder for PR-14.

use ratatui::prelude::*;
use ratatui::widgets::*;

pub fn render(frame: &mut Frame, area: Rect) {
    let placeholder = Paragraph::new("Chat (not yet implemented)")
        .block(Block::default().borders(Borders::ALL).title(" Chat "));
    frame.render_widget(placeholder, area);
}
