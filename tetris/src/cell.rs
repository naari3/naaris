use std::fmt::Display;

use termion::{color, style};

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Black,
    White,
    Red,
    Orange,
    Yellow,
    Green,
    Cyan,
    Blue,
    Purple,
    Glay,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Cell::*;
        match self {
            Black => write!(f, "{}B{}", color::Fg(color::Black), style::Reset),
            White => write!(f, "{}W{}", color::Fg(color::White), style::Reset),
            Red => write!(f, "{}R{}", color::Fg(color::Red), style::Reset),
            Orange => write!(f, "{}O{}", color::Fg(color::LightRed), style::Reset),
            Yellow => write!(f, "{}Y{}", color::Fg(color::Yellow), style::Reset),
            Green => write!(f, "{}G{}", color::Fg(color::Green), style::Reset),
            Cyan => write!(f, "{}C{}", color::Fg(color::Cyan), style::Reset),
            Blue => write!(f, "{}B{}", color::Fg(color::Blue), style::Reset),
            Purple => write!(f, "{}P{}", color::Fg(color::Magenta), style::Reset),
            Glay => write!(f, "{}G{}", color::Fg(color::LightBlack), style::Reset),
        }
    }
}
