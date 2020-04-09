
use std::collections::hash_map::{HashMap};

#[derive(PartialEq,Eq,Hash,Clone,Copy)]
pub struct Loc {
  pub row: i64,
  pub col: i64,
}

impl Loc {
  pub fn new(row: i64, col: i64) -> Self {
    Self {
      row,
      col,
    }
  }

  pub fn neighbors(&self) -> [Loc;8] {
    [
      Loc::new(self.row + 1, self.col + 1),
      Loc::new(self.row + 1, self.col - 1),
      Loc::new(self.row - 1, self.col + 1),
      Loc::new(self.row - 1, self.col - 1),
      Loc::new(self.row + 1, self.col    ),
      Loc::new(self.row    , self.col + 1),
      Loc::new(self.row - 1, self.col    ),
      Loc::new(self.row    , self.col - 1),
    ]
  }
}



pub struct World {
  buffer_1: HashMap<Loc,bool>,
  buffer_2: HashMap<Loc,bool>,
  using_buffer_1: bool,
}

impl World {

  pub fn new() -> World {
    Self {
      buffer_1: HashMap::new(),
      buffer_2: HashMap::new(),
      using_buffer_1: true,
    }
  }

  /**
   * Initialize from a configuration string. Assumes string is a grid of 
   * periods and asterisks (rows separated by line breaks), where asterisks
   * are "alive" cells and periods are dead cells.
   */
  pub fn from_configuration(data: &str, dead_char: char, alive_char: char) -> Result<Self,String> {
    let mut world = Self::new();

    let mut row = 0;
    let mut col = 0;

    for c in data.chars() {
      if c == dead_char {
        world.set(&Loc { row, col }, false);
        col += 1;
      } else if c == alive_char {
        world.set(&Loc { row, col }, true);
        col += 1;
      } else if c == '\n' {
        row += 1;
        col = 0;
      } else if c == '\r' {
        // do nothing
      } else {
        return Err(format!("Invalid char '{}' at {}, {}", c, row, col));
      }
    }

    return Ok(world);
  }

  pub fn current_buffer(&self) -> &HashMap<Loc,bool> {
    if self.using_buffer_1 { 
      &self.buffer_1 
    } else { 
      &self.buffer_2 
    }
  }

  fn next_buffer(&mut self) -> &mut HashMap<Loc,bool> {
    if self.using_buffer_1 {
      &mut self.buffer_2
    } else { 
      &mut self.buffer_1 
    }
  }

  /**
   * Get aliveness status of a location in the world.
   */
  pub fn get(&self, loc: &Loc) -> bool {
    is_alive(self.current_buffer(), loc)
  }

  /**
   * Set aliveness status of a location in the world.
   */
  pub fn set(&mut self, loc: &Loc, alive: bool) {
    let next_buffer = self.next_buffer();

    // If this location is already in the HashMap, set its value. Otherwise,
    // add it as a new entry to the HashMap.
    match next_buffer.get_mut(loc) {
      Some(val) => *val = alive,
      None => { next_buffer.insert(*loc, alive); }
    };

    if alive {
      // If this location is now alive, we need to add any of its neighbors not 
      // already in the HashMap, to it.
      for neighbor in loc.neighbors().iter() {
        if next_buffer.get(neighbor).is_none() {
          next_buffer.insert(*neighbor, false);
        }
      }
    }
  }

  /**
   * One "tick" of the world.
   */
  pub fn step(&mut self) {
    let keys: Vec<Loc> = self.current_buffer().keys().map(|&loc| loc).collect();

    for loc in keys.iter() {
      let alive: bool = self.get(&loc);
      let neighbors: [Loc;8] = loc.neighbors();
      let alive_neighbors: usize = neighbors.iter()
        .map(|neighbor| is_alive(self.current_buffer(), neighbor))
        .filter(|alive| *alive)
        .count();

      // If this cell is dead and doesn't have any alive neighbors, we don't 
      // need to check on the next cycle for whether or not it might become 
      // alive, so we can omit it altogether from the next HashMap.
      if alive_neighbors > 0 {
        self.set(&loc, new_status(alive, alive_neighbors));
      }
    }

    // Toggle buffers
    self.using_buffer_1 = !self.using_buffer_1;

    // Clear the old buffer
    self.next_buffer().clear();
  }
}

/**
 * Whether or not the supplied location is alive, based on the supplied buffer.
 */
fn is_alive(buffer: &HashMap<Loc,bool>, loc: &Loc) -> bool {
  *buffer.get(loc).unwrap_or(&false)
}


/**
 * Given the status of one cell and the number of its neighbors that are alive,
 * determine whether it's alive in the next step. This is the core rule of 
 * Conway's Game of Life.
 */
fn new_status(alive: bool, alive_neighbors: usize) -> bool {
  if alive && (alive_neighbors == 2 || alive_neighbors == 3) {
    true
  } else if !alive && alive_neighbors == 3 {
    true
  } else {
    false
  }
}
