mod example;

use std::vec;

use example::{Action, Direction, IpAddr};

fn main() {
    let direction = Direction::South;
    match direction {
        Direction::East => println!("Going East!"),
        Direction::North | Direction::South => println!("Going North or South!"),
        o => println!("Going {:?}!", o),
        // _ => println!("Going West!"),
    };

    let ip = IpAddr::Ipv6;
    let ip_str = match ip {
        IpAddr::Ipv4 => "127.0.0.1",
        IpAddr::Ipv6 => "::1",
    };
    println!("IP Address: {}", ip_str);

    let actions = [
        Action::Say("Hello".to_string()),
        Action::Move { x: 10, y: 20 },
        Action::ChangeColor(255, 255, 255),
    ];

    for action in actions {
        match action {
            Action::Say(msg) => {
                println!("Saying: {}", msg);
            }
            Action::Move { x, y } => {
                println!("Moving to (x: {}, y: {})", x, y);
            }
            Action::ChangeColor(r, g, b) => {
                println!("Changing color to RGB({}, {}, {})", r, g, b);
            }
        }
    }

    let some_value = Some(43u8);
    if let Some(43) = some_value {
        println!("Found the value 43!");
    }

    let v = vec![
        Direction::East,
        Direction::West,
        Direction::North,
        Direction::North,
        Direction::South,
    ];
    let directions = v
        .iter()
        .filter(|x| matches!(x, Direction::East | Direction::North));
    for direction in directions {
        println!("Found direction: {:?}", direction);
    }

    let result = matches!(Some(40u8), Some(x) if x > 5);
    println!("Result: {}", result);
}
