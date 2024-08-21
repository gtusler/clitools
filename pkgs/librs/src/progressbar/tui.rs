use super::{
    terminal_helpers::get_terminal_dimensions, tui_config::ProgressBarTuiConfig,
    view::ProgressBarView,
};
use std::io::{self, Stdout};
use terminal::{error, Action, Clear};

/// Takes control of the whole terminal screen and offers a nice interface focontrolling ui
/// element.
pub struct ProgressBarTui {
    config: ProgressBarTuiConfig,
    terminal: terminal::Terminal<Stdout>,
    terminal_dimensions: (u16, u16),
    view: ProgressBarView,
    current: usize,
    pub completed: bool,
}

impl ProgressBarTui {
    /// Creates a new tui, using default config values
    pub fn new() -> ProgressBarTui {
        let terminal = terminal::stdout();
        let terminal_dimensions = get_terminal_dimensions(&terminal).unwrap();
        let config = ProgressBarTuiConfig::default();

        ProgressBarTui {
            completed: false,
            config,
            current: 0,
            terminal,
            terminal_dimensions,
            view: ProgressBarView {
                max_steps: config.max,
                style: config.bar_style,
                width: terminal_dimensions.0,
            },
        }
    }

    pub fn with(config: ProgressBarTuiConfig) -> ProgressBarTui {
        let terminal = terminal::stdout();
        let terminal_dimensions = get_terminal_dimensions(&terminal).unwrap();

        ProgressBarTui {
            completed: false,
            config,
            current: 0,
            terminal,
            terminal_dimensions,
            view: ProgressBarView {
                style: config.bar_style,
                max_steps: config.max,
                width: terminal_dimensions.0,
            },
        }
    }

    /// Clear the terminal screen
    /// Create view areas
    pub fn start(&self) -> error::Result<()> {
        println!("start");
        // println!("config: {}", self.config);
        // println!("dimensions: {:?}", self.terminal_dimensions);

        self.terminal.act(Action::EnterAlternateScreen)?;
        self.terminal.act(Action::HideCursor)?;
        self.terminal
            .act(Action::MoveCursorTo(0, self.terminal_dimensions.1))?;

        Ok(())
    }

    /// Return the terminal to its initial state
    pub fn stop(&self) -> error::Result<()> {
        self.terminal.act(Action::LeaveAlternateScreen)?;
        self.terminal.act(Action::ShowCursor)?;

        println!("stop");

        Ok(())
    }

    /// Increment internal counters, progress to the next step
    pub fn tick(&mut self, msg: Option<String>) -> error::Result<()> {
        self.current = self.current + self.config.step;

        let bar = self.view.draw(self.current);

        // first, we clear the bar
        self.terminal
            .act(Action::MoveCursorTo(0, self.terminal_dimensions.1 - 2))?;
        self.terminal
            .act(Action::ClearTerminal(Clear::CurrentLine))?;

        // then print the message
        if let Some(m) = msg {
            println!("{}", m);
        }
        print!("\n");

        // then reprint the bar
        self.terminal
            .act(Action::MoveCursorTo(0, self.terminal_dimensions.1 - 2))?;
        println!("{}", bar);

        // and check for fin
        if self.current >= self.config.max && self.config.pause_on_fin {
            self.completed = true;
            return self.pause_for_fin();
        }

        Ok(())
    }

    pub fn pause_for_fin(&self) -> error::Result<()> {
        println!("Finished. Press Enter to quit...");

        let stdin = io::stdin();
        let input = &mut String::new();
        input.clear();
        let _ = stdin.read_line(input);

        if input.len() >= 1 {
            return self.stop();
        }

        Ok(())
    }
}
