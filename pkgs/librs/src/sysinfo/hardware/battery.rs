use std::fmt::Display;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BatteryInfo {
    pub idx: usize,
    pub vendor: Option<String>,
    pub model: Option<String>,
    pub state_of_health: String,
    pub state: State,
    pub technology: Technology,
    pub temperature: Option<String>,
}

impl BatteryInfo {
    pub fn print(&self) -> () {
        println!("#{}", self.idx);
        if let Some(vendor) = &self.vendor {
            println!("Vendor: {}", vendor);
        }
        if let Some(model) = &self.model {
            println!("Model: {}", model);
        }
        println!("Tech: {}", self.technology);
        println!("State of health: {}", self.state_of_health);
        println!("State: {}", self.state);
        if let Some(temp) = &self.temperature {
            println!("Temp: {}", temp);
        }
    }
}

impl Display for BatteryInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines: Vec<String> = vec![format!("#{}", self.idx)];
        if let Some(vendor) = &self.vendor {
            lines.push(format!("Vendor: {}", vendor));
        }
        if let Some(model) = &self.model {
            lines.push(format!("Model: {}", model));
        }
        lines.push(format!("Tech: {}", self.technology));
        lines.push(format!("State of health: {}", self.state_of_health));
        lines.push(format!("State: {}", self.state));
        if let Some(temp) = &self.temperature {
            lines.push(format!("Temp: {}", temp));
        }
        let lines = lines.join("\n");

        write!(f, "{}", lines)
    }
}

#[derive(Debug, Serialize)]
pub enum State {
    Unknown,
    Charging,
    Discharging,
    Empty,
    Full,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match &self {
            State::Unknown => "Unknown",
            State::Charging => "Charging",
            State::Discharging => "Discharging",
            State::Empty => "Empty",
            State::Full => "Full",
        };
        write!(f, "{}", msg)
    }
}

impl From<battery::State> for State {
    fn from(value: battery::State) -> Self {
        match value {
            battery::State::Unknown => State::Unknown,
            battery::State::Charging => State::Charging,
            battery::State::Discharging => State::Discharging,
            battery::State::Empty => State::Empty,
            battery::State::Full => State::Full,
            _ => State::Unknown,
        }
    }
}

#[derive(Debug, Serialize)]
pub enum Technology {
    Unknown,
    LithiumIon,
    LeadAcid,
    LithiumPolymer,
    NickelMetalHydride,
    NickelCadmium,
    NickelZinc,
    LithiumIronPhosphate,
    RechargeableAlkalineManganese,
}

impl Display for Technology {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Technology::Unknown => "Unknown",
            Technology::LithiumIon => "Lithium Ion",
            Technology::LeadAcid => "Lead Acid",
            Technology::LithiumPolymer => "Lithium Polymer",
            Technology::NickelMetalHydride => "Nickel Metal Hydride",
            Technology::NickelCadmium => "Nickel Cadmium",
            Technology::NickelZinc => "Nickel Zinc",
            Technology::LithiumIronPhosphate => "Lithium Iron Phosphate",
            Technology::RechargeableAlkalineManganese => "Rechargeable Alkaline Manganese",
        };
        write!(f, "{}", msg)
    }
}

impl From<battery::Technology> for Technology {
    fn from(value: battery::Technology) -> Self {
        match value {
            battery::Technology::Unknown => Technology::Unknown,
            battery::Technology::LithiumIon => Technology::LithiumIon,
            battery::Technology::LeadAcid => Technology::LeadAcid,
            battery::Technology::LithiumPolymer => Technology::LithiumPolymer,
            battery::Technology::NickelMetalHydride => Technology::NickelMetalHydride,
            battery::Technology::NickelCadmium => Technology::NickelCadmium,
            battery::Technology::NickelZinc => Technology::NickelZinc,
            battery::Technology::LithiumIronPhosphate => Technology::LithiumIronPhosphate,
            battery::Technology::RechargeableAlkalineManganese => Technology::RechargeableAlkalineManganese,
            _ => Technology::Unknown,
        }
    }
}

pub fn get_battery_info() -> Vec<BatteryInfo> {
    let manager = battery::Manager::new().expect("Failed to create battery manager");
    let mut output = vec![];

    let batteries = manager.batteries().expect("there are no batteries");

    for (idx, maybe_battery) in batteries.enumerate() {
        let battery = maybe_battery.expect("Failed to enumerate battery");

        let vendor = match battery.vendor() {
            Some(vendor_str) => Some(vendor_str.to_string()),
            None => None,
        };
        let model = match battery.model() {
            Some(model_str) => Some(model_str.to_string()),
            None => None,
        };
        let temperature = match battery.temperature() {
            Some(temp) => Some(format!("{:?}", temp)),
            None => None,
        };

        let state_of_health = format!("{:?}", battery.state_of_health());

        let battery_info = BatteryInfo {
            idx,
            vendor,
            model,
            state_of_health,
            state: State::from(battery.state()),
            technology: Technology::from(battery.technology()),
            temperature,
        };

        output.push(battery_info);
    }

    output
}

pub fn print_batteries_info(batteries: Vec<BatteryInfo>) -> () {
    let suffix = if batteries.len() == 1 { "y" } else { "ies" };
    println!("{} batter{}", batteries.len(), suffix);

    for bat in batteries {
        bat.print();
    }
}
