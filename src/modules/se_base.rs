#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Straight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    FromSouth,
    FromNorth,
    FromWest,
    FromEast,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Light {
    Green,
    Red,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrafficLight {
    pub color: Light,
}
