//! Config tab -- placeholder for PR-15.

use ratatui::prelude::*;
use ratatui::widgets::*;

pub fn render(frame: &mut Frame, area: Rect) {
    let placeholder = Paragraph::new("Config (not yet implemented)")
        .block(Block::default().borders(Borders::ALL).title(" Config "));
    frame.render_widget(placeholder, area);
}
