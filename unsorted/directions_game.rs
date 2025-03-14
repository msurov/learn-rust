enum Direction {
  Up,
  Down,
  Left,
  Right,
  Undefined
}

fn parse_direction(s: &str) -> Direction {
  return match s {
    "Up" => Direction::Up,
    "Down" => Direction::Down,
    "Left" => Direction::Left,
    "Right" => Direction::Right,
    _ => Direction::Undefined
  }
}

fn direction_to_string(dir : &Direction) -> &str {
  return match dir {
    Direction::Up => "Up",
    Direction::Down => "Down",
    Direction::Left => "Left",
    Direction::Right => "Right",
    _ => "Undef"
  }
}

fn get_opposite_direction(dir: Direction) -> Direction {
  return match dir {
    Direction::Up => Direction::Down,
    Direction::Down => Direction::Up,
    Direction::Left => Direction::Right,
    Direction::Right => Direction::Left,
    _ => Direction::Undefined
  }
}

fn main() {
  let mut dir = String::new();
  std::io::stdin().read_line(&mut dir).expect("Can't read direction");
  let dir = parse_direction(dir.trim());
  let opdir = get_opposite_direction(dir);
  println!("{}", direction_to_string(&opdir));
}
