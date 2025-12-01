use std::env;
use std::fs;
use std::process;

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct Rotation {
    direction: Direction,
    amount: u32,
}

impl Rotation {
    fn parse(line: &str) -> Result<Self, String> {
        let line = line.trim();
        if line.is_empty() {
            return Err("Empty line".to_string());
        }

        let direction = match line.chars().next() {
            Some('L') => Direction::Left,
            Some('R') => Direction::Right,
            _ => return Err(format!("Invalid direction: {}", line)),
        };

        let amount_str = &line[1..];
        let amount = amount_str
            .parse::<u32>()
            .map_err(|_| format!("Invalid number: {}", amount_str))?;

        Ok(Rotation { direction, amount })
    }
}

fn parse_input(content: &str) -> Vec<Rotation> {
    content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| match Rotation::parse(line) {
            Ok(rotation) => Some(rotation),
            Err(e) => {
                eprintln!("Warning: Failed to parse line '{}': {}", line, e);
                None
            }
        })
        .collect()
}

// Returns the new dial position and how many times the "pointer" was on zero
fn rotate_dial(current: u32, rotation: &Rotation, dial_size: u32) -> (u32, u32) {
    match rotation.direction {
        Direction::Left => {
            // When rotating left, we count how many times we pass position 0
            let times_crossed = if rotation.amount < current {
                0
            } else if current == 0 {
                // Starting at 0: only count if we do a full rotation or more
                rotation.amount / dial_size
            } else {
                // rotation.amount >= current and current > 0
                // Use max of 1 or the number of full cycles past zero
                u32::max(1, (rotation.amount - current) / dial_size)
            };
            
            // Calculate new position
            let new_position = if rotation.amount > current {
                let remainder = (rotation.amount - current) % dial_size;
                if remainder == 0 {
                    0
                } else {
                    dial_size - remainder
                }
            } else {
                current - rotation.amount
            };
            
            (new_position, times_crossed)
        }
        Direction::Right => {
            let new_position = (current + rotation.amount) % dial_size;
            let times_crossed = (current + rotation.amount) / dial_size;
            (new_position, times_crossed)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let content = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", filename, err);
        process::exit(1);
    });

    let rotations = parse_input(&content);
    let mut dial_position = 50;
    let mut vault_password = 0;

    println!("Parsed {} rotations:", rotations.len());
    for rotation in &rotations {
        print!("Start {}, then rotate {:?} {}", dial_position, rotation.direction, rotation.amount);
        let (new_position, hit_zero_times) = rotate_dial(dial_position, rotation, 100);
        dial_position = new_position;
        println!(" to {} hitting zero {} times", dial_position, hit_zero_times);

        // if dial_position == 0 {
        //     vault_password += 1;
        // }
        vault_password += hit_zero_times;
    }

    println!("Vault password: {}", vault_password);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_left() {
        let rotation = Rotation::parse("L68").unwrap();
        assert_eq!(rotation.direction, Direction::Left);
        assert_eq!(rotation.amount, 68);
    }

    #[test]
    fn test_parse_right() {
        let rotation = Rotation::parse("R48").unwrap();
        assert_eq!(rotation.direction, Direction::Right);
        assert_eq!(rotation.amount, 48);
    }

    #[test]
    fn test_parse_invalid_direction() {
        assert!(Rotation::parse("X10").is_err());
    }

    #[test]
    fn test_parse_invalid_number() {
        assert!(Rotation::parse("Labc").is_err());
    }

    #[test]
    fn test_parse_input() {
        let input = "L68\nR48\nL5\n";
        let rotations = parse_input(input);
        assert_eq!(rotations.len(), 3);
        assert_eq!(rotations[0].direction, Direction::Left);
        assert_eq!(rotations[0].amount, 68);
        assert_eq!(rotations[1].direction, Direction::Right);
        assert_eq!(rotations[1].amount, 48);
    }

    #[test]
    fn test_rotate_dial() {
        let dial_size = 100;
        let current = 30;
        let rotation_left = Rotation {
            direction: Direction::Left,
            amount: 5,
        };
        let rotation_right = Rotation {
            direction: Direction::Right,
            amount: 5,
        };
        assert_eq!(rotate_dial(current, &rotation_left, dial_size), (25, 0));
        assert_eq!(rotate_dial(current, &rotation_right, dial_size), (35, 0));
    }

    #[test]
    fn test_rotate_dial_wrap_around() {
        let dial_size = 100;
        let current = 2;
        let rotation_left = Rotation {
            direction: Direction::Left,
            amount: 5,
        };
        let rotation_right = Rotation {
            direction: Direction::Right,
            amount: 99,
        };
        assert_eq!(rotate_dial(current, &rotation_left, dial_size), (97, 1));
        assert_eq!(rotate_dial(current, &rotation_right, dial_size), (1, 1));
    }

    #[test]
    fn test_rotate_to_zero() {
        let dial_size = 100;
        let current = 50;
        let rotation_left = Rotation {
            direction: Direction::Left,
            amount: 50,
        };
        let rotation_right = Rotation {
            direction: Direction::Right,
            amount: 50,
        };
        assert_eq!(rotate_dial(current, &rotation_left, dial_size), (0, 1));
        assert_eq!(rotate_dial(current, &rotation_right, dial_size), (0, 1));
    }

    #[test]
    fn test_rotate_past_zero_multiple_times() {
        let dial_size = 100;
        let current = 10;
        let rotation_left = Rotation {
            direction: Direction::Left,
            amount: 250,
        };
        let rotation_right = Rotation {
            direction: Direction::Right,
            amount: 350,
        };
        assert_eq!(rotate_dial(current, &rotation_left, dial_size), (60, 2));
        assert_eq!(rotate_dial(current, &rotation_right, dial_size), (60, 3));
    }
}
