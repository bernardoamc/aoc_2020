use std::io::{self, Read};

pub struct Ship {
    angle: i32,
    x: i32,
    y: i32,
    waypoint_x: i32,
    waypoint_y: i32,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            angle: 0,
            x: 0,
            y: 0,
            waypoint_x: 10,
            waypoint_y: 1,
        }
    }

    pub fn apply_without_waypoint(&mut self, command: &char, value: i32) {
        match &command {
            'R' | 'L' => self.turn(command, value),
            'F' => match self.angle {
                0 => self.advance(&'E', value),
                90 => self.advance(&'N', value),
                180 => self.advance(&'W', value),
                270 => self.advance(&'S', value),
                _ => panic!("Invalid angle"),
            },
            _ => self.advance(command, value),
        }
    }

    pub fn advance(&mut self, direction: &char, distance: i32) {
        match &direction {
            'E' => self.x += distance,
            'W' => self.x -= distance,
            'N' => self.y += distance,
            'S' => self.y -= distance,
            _ => panic!("Invalid movement"),
        }
    }

    pub fn turn(&mut self, turn: &char, degrees: i32) {
        match &turn {
            'R' => self.angle = (self.angle + (360 - degrees)) % 360,
            'L' => self.angle = (self.angle + degrees) % 360,
            _ => panic!("Invalid turn"),
        }
    }

    pub fn apply_with_waypoint(&mut self, command: &char, value: i32) {
        match &command {
            'R' | 'L' => self.rotate_waypoint(command, value),
            'F' => {
                self.x += self.waypoint_x * value;
                self.y += self.waypoint_y * value;
            }
            _ => self.advance_waypoint(command, value),
        }
    }

    pub fn advance_waypoint(&mut self, direction: &char, distance: i32) {
        match &direction {
            'E' => self.waypoint_x += distance,
            'W' => self.waypoint_x -= distance,
            'N' => self.waypoint_y += distance,
            'S' => self.waypoint_y -= distance,
            _ => panic!("Invalid movement"),
        }
    }

    pub fn rotate_waypoint(&mut self, turn: &char, degrees: i32) {
        let rotation = match &turn {
            'R' => (360 - degrees) % 360,
            'L' => degrees % 360,
            _ => 0,
        };

        let (x, y) = (self.waypoint_x, self.waypoint_y);

        match rotation {
            0 => (),
            90 => {
                self.waypoint_x = -y;
                self.waypoint_y = x;
            }
            180 => {
                self.waypoint_x = -x;
                self.waypoint_y = -y;
            }
            270 => {
                self.waypoint_x = y;
                self.waypoint_y = -x;
            }
            _ => panic!("Invalid rotation"),
        }
    }
}

pub fn part1(input: &str) -> i32 {
    let mut ship = Ship::new();

    for line in input.lines() {
        let command = line[..1].parse::<char>().unwrap();
        let value = line[1..].parse::<i32>().unwrap();
        ship.apply_without_waypoint(&command, value);
    }

    ship.x.abs() + ship.y.abs()
}

pub fn part2(input: &str) -> i32 {
    let mut ship = Ship::new();

    for line in input.lines() {
        let command = line[..1].parse::<char>().unwrap();
        let value = line[1..].parse::<i32>().unwrap();
        ship.apply_with_waypoint(&command, value);
    }

    ship.x.abs() + ship.y.abs()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
}
