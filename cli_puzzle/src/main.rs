use rand::seq::SliceRandom;
use rand::rng;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::io::{self, Write};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn all() -> Vec<Direction> {
        vec![Direction::North, Direction::South, Direction::East, Direction::West]
    }
}

struct Maze {
    width: usize,
    height: usize,
    visited: HashSet<Position>,
    connections: HashSet<(Position, Position)>,
    start: Position,
    end: Position,
    player: Position,
}

impl Maze {
    fn new(width: usize, height: usize) -> Self {
        let start = Position { x: 0, y: 0 };
        let end = Position { x: width - 1, y: height - 1 };
        let mut maze = Maze {
            width,
            height,
            visited: HashSet::new(),
            connections: HashSet::new(),
            start,
            end,
            player: start,
        };
        maze.generate_iterative();
        maze
    }

    fn generate_iterative(&mut self) {
        let mut rng = rand::rng();
        let mut stack = VecDeque::new();
        stack.push_back(self.start);
        self.visited.insert(self.start);

        while let Some(pos) = stack.pop_back() {
            let mut directions = Direction::all();
            directions.shuffle(&mut rng);

            for dir in directions {
                if let Some(next_pos) = self.move_pos(pos, dir) {
                    if !self.visited.contains(&next_pos) {
                        self.connections.insert((pos, next_pos));
                        self.connections.insert((next_pos, pos));
                        self.visited.insert(next_pos);
                        stack.push_back(next_pos);
                    }
                }
            }
        }
    }

    fn move_pos(&self, pos: Position, dir: Direction) -> Option<Position> {
        match dir {
            Direction::North => {
                if pos.y > 0 {
                    Some(Position { x: pos.x, y: pos.y - 1 })
                } else {
                    None
                }
            }
            Direction::South => {
                if pos.y < self.height - 1 {
                    Some(Position { x: pos.x, y: pos.y + 1 })
                } else {
                    None
                }
            }
            Direction::East => {
                if pos.x < self.width - 1 {
                    Some(Position { x: pos.x + 1, y: pos.y })
                } else {
                    None
                }
            }
            Direction::West => {
                if pos.x > 0 {
                    Some(Position { x: pos.x - 1, y: pos.y })
                } else {
                    None
                }
            }
        }
    }

    fn try_move(&mut self, dir: Direction) -> bool {
        if let Some(new_pos) = self.move_pos(self.player, dir) {
            if self.connections.contains(&(self.player, new_pos)) {
                self.player = new_pos;
                return true;
            }
        }
        false
    }

    fn is_at_end(&self) -> bool {
        self.player == self.end
    }

    fn display(&self) {
        println!("\nMaze Legend: P=Player, E=Exit, . = Path, # = Wall\n");
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Position { x, y };
                if pos == self.player {
                    print!("P");
                } else if pos == self.end {
                    print!("E");
                } else {
                    print!(".");
                }

                if x < self.width - 1 {
                    let east = Position { x: x + 1, y };
                    if self.connections.contains(&(pos, east)) {
                        print!(" ");
                    } else {
                        print!("|");
                    }
                }
            }
            println!();

            if y < self.height - 1 {
                for x in 0..self.width {
                    let pos = Position { x, y };
                    let south = Position { x, y: y + 1 };
                    if self.connections.contains(&(pos, south)) {
                        print!("  ");
                    } else {
                        print!("--");
                    }
                }
                println!();
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let show_display = args.iter().any(|arg| arg == "--show-maze");

    let mut maze = Maze::new(1_000, 1_000); // Change size here
    println!("Welcome to the Maze Puzzle!");
    println!("Navigate using: n (north), s (south), e (east), w (west), q (quit)");

    loop {
        if show_display {
            maze.display();
        }

        print!("You are at ({}, {}). Move: ", maze.player.x, maze.player.y);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "n" => { maze.try_move(Direction::North); },
            "s" => { maze.try_move(Direction::South); },
            "e" => { maze.try_move(Direction::East); },
            "w" => { maze.try_move(Direction::West); },
            "q" => {
                println!("Quitting the maze. Goodbye!");
                break;
            },
            _ => {
                println!("Unknown command.");
                continue;
            }
        }

        if maze.is_at_end() {
            println!("Congratulations! You found the exit!");
            println!("Confirmation code: blorp");
            break;
        }
    }
}
