#[allow(dead_code)]
#[derive(Debug)]
pub enum Direction {
    East,
    West,
    North,
    South,
}

#[allow(dead_code)]
pub enum IpAddr {
    Ipv4,
    Ipv6,
}

pub enum Action {
    Say(String),
    Move { x: i32, y: i32 },
    ChangeColor(i32, i32, i32),
}
