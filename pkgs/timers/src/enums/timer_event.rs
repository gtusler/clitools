use std::fmt::Display;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum TimerEvent {
    Start,
    Stop,
    Invalid,
}

impl TimerEvent {
    pub fn from_str(value: &str) -> TimerEvent {
        match value {
            "start" => TimerEvent::Start,
            "stop" => TimerEvent::Stop,
            _ => TimerEvent::Invalid,
        }
    }
}

impl Display for TimerEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::Start => "start",
            Self::Stop => "stop",
            Self::Invalid => "invalid",
        };

        write!(f, "{}", msg)
    }
}
