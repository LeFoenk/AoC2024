use std::fs;

fn main() {
    let file_path = "input.txt";

    let content = fs::read_to_string(file_path).unwrap();

    let mut room: Vec<Vec<Point>> = vec![vec![]];

    let mut guard = Guard {
        pos_x: 0,
        pos_y: 0,
        direction: Direction::Top,
    };

    let mut x = 0;
    let mut y = 0;
    for character in content.chars() {
        match character {
            '.' => {
                room[y].push(Point::Empty);
                x += 1;
            }
            '#' => {
                room[y].push(Point::Blocked);
                x += 1;
            }
            '^' => {
                guard.pos_x = x;
                guard.pos_y = y;
                guard.direction = Direction::Top;
                room[y].push(Point::Visited {
                    direction: guard.direction.clone(),
                });
                x += 1;
            }
            '>' => {
                guard.pos_x = x;
                guard.pos_y = y;
                guard.direction = Direction::Right;
                room[y].push(Point::Visited {
                    direction: guard.direction.clone(),
                });
                x += 1;
            }
            'V' => {
                guard.pos_x = x;
                guard.pos_y = y;
                guard.direction = Direction::Bot;
                room[y].push(Point::Visited {
                    direction: guard.direction.clone(),
                });
                x += 1;
            }
            '<' => {
                guard.pos_x = x;
                guard.pos_y = y;
                guard.direction = Direction::Left;
                room[y].push(Point::Visited {
                    direction: guard.direction.clone(),
                });
                x += 1;
            }
            '\n' => {
                y += 1;
                x = 0;
                room.push(vec![]);
            }
            _ => {
                panic!("Wrong character in input: {}", character);
            }
        }
    }

    if room[room.len() - 1].is_empty() {
        room.pop();
    }

    let room_start = room.clone();
    let guard_start = guard.clone();

    let mut visited: Vec<(usize, usize)> = vec![];
    while let Some((x, y)) = guard.infront() {
        if let Some(point) = room.get(y).and_then(|row: &Vec<Point>| row.get(x)) {
            match point {
                Point::Visited { direction: _ } => {
                    guard.walk();
                }
                Point::Empty => {
                    room[y][x] = Point::Visited {
                        direction: guard.direction.clone(),
                    };
                    guard.walk();
                    visited.push((x, y));
                }
                Point::Blocked => {
                    guard.turn();
                }
            };
        } else {
            break;
        }
    }

    let mut result = visited.len();
    println!("Part 1: {}", result);

    result = 0;
    for (a, b) in visited {
        guard = guard_start.clone();
        room = room_start.clone();
        room[b][a] = Point::Blocked;
        while let Some((x, y)) = guard.infront() {
            if let Some(point) = room.get(y).and_then(|row: &Vec<Point>| row.get(x)) {
                match point {
                    Point::Blocked => {
                        guard.turn();
                    }
                    Point::Empty => {
                        room[y][x] = Point::Visited {
                            direction: guard.direction.clone(),
                        };
                        guard.walk();
                    }
                    Point::Visited { direction } => {
                        if *direction == guard.direction {
                            result += 1;
                            break;
                        } else {
                            room[y][x] = Point::Visited {
                                direction: guard.direction.clone(),
                            };
                            guard.walk();
                        }
                    }
                }
            } else {
                break;
            }
        }
    }

    println!("Part 2: {}", result);
}

#[derive(PartialEq, Clone)]
enum Point {
    Empty,
    Blocked,
    Visited { direction: Direction },
}

#[derive(Clone)]
struct Guard {
    pos_x: usize,
    pos_y: usize,
    direction: Direction,
}

impl Guard {
    fn walk(&mut self) {
        match self.direction {
            Direction::Top => {
                self.pos_y -= 1;
            }
            Direction::Bot => {
                self.pos_y += 1;
            }
            Direction::Left => {
                self.pos_x -= 1;
            }
            Direction::Right => {
                self.pos_x += 1;
            }
        }
    }

    fn turn(&mut self) {
        match self.direction {
            Direction::Top => self.direction = Direction::Right,
            Direction::Right => self.direction = Direction::Bot,
            Direction::Bot => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Top,
        };
    }

    fn infront(&self) -> Option<(usize, usize)> {
        match self.direction {
            Direction::Top => {
                if self.pos_y > 0 {
                    return Some((self.pos_x, self.pos_y - 1));
                }
            }
            Direction::Right => {
                return Some((self.pos_x + 1, self.pos_y));
            }
            Direction::Bot => {
                return Some((self.pos_x, self.pos_y + 1));
            }
            Direction::Left => {
                if self.pos_x > 0 {
                    return Some((self.pos_x - 1, self.pos_y));
                }
            }
        }
        return None;
    }
}

#[derive(PartialEq, Clone)]
enum Direction {
    Top,
    Bot,
    Left,
    Right,
}
