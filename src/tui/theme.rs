//! ZeroClaw TUI color theme (Lobster palette).

use ratatui::style::Color;

/// Primary accent color (lobster red)
pub const ACCENT: Color = Color::Rgb(220, 50, 47);

/// Secondary accent
pub const ACCENT_SECONDARY: Color = Color::Rgb(38, 139, 210);

/// Success / healthy
pub const SUCCESS: Color = Color::Rgb(133, 153, 0);

/// Warning
pub const WARNING: Color = Color::Rgb(181, 137, 0);

/// Error / danger
pub const ERROR: Color = Color::Rgb(220, 50, 47);

/// Primary foreground
pub const FG: Color = Color::Rgb(253, 246, 227);

/// Dimmed foreground
pub const FG_DIM: Color = Color::Rgb(147, 161, 161);

/// Primary background
pub const BG: Color = Color::Rgb(0, 43, 54);

/// Surface (slightly lighter than background)
pub const SURFACE: Color = Color::Rgb(7, 54, 66);

/// Border color
pub const BORDER: Color = Color::Rgb(88, 110, 117);
