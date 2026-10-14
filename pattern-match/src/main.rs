mod data;

use data::Directions;

fn main() {
    /* match pattern */
    let up = Directions::Up;

    match up {
        Directions::Up => println!("Going up!"),
        Directions::Down => println!("Going down!"),
        _ => println!("Going left or right!"),
    }

    /* if let pattern */

    let value = Some(Directions::Left);

    if let Some(x) = value {
        println!("Matched! {:?}", x);
    }

    /* while let pattern */

    let mut stack = vec![1, 2, 3];
    while let Some(v) = stack.pop() {
        println!("Popped: {}", v);
    }
}
