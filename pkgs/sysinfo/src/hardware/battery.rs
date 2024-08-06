pub struct BatteryInfo {
    pub idx: usize,
    pub vendor: Option<String>,
    pub model: Option<String>,
    pub state_of_health: String,
    pub state: battery::State,
    pub technology: battery::Technology,
    pub temperature: Option<String>,
}

impl BatteryInfo {
    pub fn print(&self) -> () {
        println!("Battery #{}", self.idx);
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
            state: battery.state(),
            technology: battery.technology(),
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
