//! Channels tab -- placeholder for PR-13.

use ratatui::prelude::*;
use ratatui::widgets::*;

pub fn render(frame: &mut Frame, area: Rect) {
    let placeholder = Paragraph::new("Channels (not yet implemented)")
        .block(Block::default().borders(Borders::ALL).title(" Channels "));
    frame.render_widget(placeholder, area);
}
