mod example;

use example::Direction;

fn main() {
    let direction = Direction::East;
    match direction {
        Direction::East => println!("Going East!"),
        Direction::North | Direction::South => println!("Going North or South!"),
        _ => println!("Going West!"),
    };
}
