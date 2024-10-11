use clap::builder::{styling::AnsiColor, Styles};

pub fn cli_style() -> Styles {
    Styles::styled()
        .header(AnsiColor::White.on_default().bold())
        .usage(AnsiColor::White.on_default().bold())
        .literal(AnsiColor::Yellow.on_default())
        .placeholder(AnsiColor::Cyan.on_default())
        .error(AnsiColor::Red.on_default().bold())
        .invalid(AnsiColor::Red.on_default().bold())
        .valid(AnsiColor::Green.on_default().bold())
}
