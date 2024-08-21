use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub enum BarWidth {
    Full,
    Fixed(usize),
}

impl Display for BarWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            BarWidth::Full => String::from("Full"),
            BarWidth::Fixed(width) => format!("Fixed {} blocks", width),
        };

        write!(f, "{}", msg)
    }
}
