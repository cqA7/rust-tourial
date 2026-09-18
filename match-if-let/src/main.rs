mod example;

use example::{Action, Direction, IpAddr};

fn main() {
    let direction = Direction::West;
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
}
