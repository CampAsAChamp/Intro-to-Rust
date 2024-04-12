#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn print_direction(direction: Direction) {
    match direction {
        Direction::Up => println!("Up"),
        Direction::Down => println!("Down"),
        Direction::Left => println!("Left"),
        Direction::Right => println!("Right"),
        _ => println!("Invalid direction"),
    }
}

fn main() {
    let up = Direction::Up;
    let down = Direction::Down;
    let left = Direction::Left;
    let right = Direction::Right;

    print_direction(up);
    print_direction(down);
    print_direction(left);
    print_direction(right);
}
