use rand::seq::SliceRandom;
use rand::rng;
use std::collections::{HashSet, VecDeque};
use std::ffi::c_void;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

pub struct Maze {
    width: usize,
    height: usize,
    visited: HashSet<Position>,
    connections: HashSet<(Position, Position)>,
    start: Position,
    end: Position,
    pub player: Position,
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
        let mut rng = rand::thread_rng();
        let mut stack = VecDeque::new();
        stack.push_back(self.start);
        self.visited.insert(self.start);

        while let Some(pos) = stack.pop_back() {
            let mut directions = [Direction::North, Direction::South, Direction::East, Direction::West];
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
            Direction::North if pos.y > 0 => Some(Position { x: pos.x, y: pos.y - 1 }),
            Direction::South if pos.y < self.height - 1 => Some(Position { x: pos.x, y: pos.y + 1 }),
            Direction::East if pos.x < self.width - 1 => Some(Position { x: pos.x + 1, y: pos.y }),
            Direction::West if pos.x > 0 => Some(Position { x: pos.x - 1, y: pos.y }),
            _ => None,
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
}

// === FFI Wrappers ===

#[no_mangle]
pub extern "C" fn maze_create(width: usize, height: usize) -> *mut Maze {
    Box::into_raw(Box::new(Maze::new(width, height)))
}

#[no_mangle]
pub extern "C" fn maze_destroy(ptr: *mut Maze) {
    if !ptr.is_null() {
        unsafe { drop(Box::from_raw(ptr)) }
    }
}

#[no_mangle]
pub extern "C" fn maze_get_player_position(ptr: *const Maze) -> Position {
    unsafe { (*ptr).player }
}

#[no_mangle]
pub extern "C" fn maze_get_end_position(ptr: *const Maze) -> Position {
    unsafe { (*ptr).end }
}

#[no_mangle]
pub extern "C" fn maze_try_move(ptr: *mut Maze, dir: Direction) -> bool {
    unsafe { (*ptr).try_move(dir) }
}

#[no_mangle]
pub extern "C" fn maze_is_at_end(ptr: *const Maze) -> bool {
    unsafe { (*ptr).is_at_end() }
}
