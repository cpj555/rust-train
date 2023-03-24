#[derive(Debug)]
pub enum TrafficSignal {
    Red,
    Green,
    Yellow,
}

pub trait GetTime {
    fn time(self: &Self) -> u64;
}

impl GetTime for TrafficSignal {
    fn time(self: &Self) -> u64 {
        match self {
            TrafficSignal::Red => 30,
            TrafficSignal::Green => 60,
            TrafficSignal::Yellow => 3,
        }
    }
}