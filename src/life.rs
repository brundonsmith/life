
use std::collections::hash_map::{HashMap};

#[derive(PartialEq,Eq,Hash,Clone,Copy)]
pub struct Loc {
  pub row: i64,
  pub col: i64,

  //neighbors: Box<[Loc;8]>
}

impl Loc {

  pub fn new(row: i64, col: i64) -> Self {
    Self {
      row,
      col,
    }
  }
}

fn neighbors(loc: &Loc) -> [Loc;8] {
  [
    Loc::new(loc.row + 1, loc.col + 1),
    Loc::new(loc.row + 1, loc.col - 1),
    Loc::new(loc.row - 1, loc.col + 1),
    Loc::new(loc.row - 1, loc.col - 1),
    Loc::new(loc.row + 1, loc.col    ),
    Loc::new(loc.row    , loc.col + 1),
    Loc::new(loc.row - 1, loc.col    ),
    Loc::new(loc.row    , loc.col - 1),
  ]
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

  pub fn from_data(data: &str, dead_char: char, alive_char: char) -> Result<Self,String> {
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

  pub fn get(&self, loc: &Loc) -> bool {
    get(self.current_buffer(), loc)
  }

  pub fn set(&mut self, loc: &Loc, alive: bool) {
    let next_buffer = self.next_buffer();

    match next_buffer.get_mut(loc) {
      Some(val) => *val = alive,
      None => { next_buffer.insert(*loc, alive); }
    };

    if alive {
      for neighbor in neighbors(loc).iter() {
        if next_buffer.get(neighbor).is_none() {
          next_buffer.insert(*neighbor, false);
        }
      }
    }
  }

  pub fn step(&mut self) {
    let keys: Vec<Loc> = self.current_buffer().keys().map(|&loc| loc).collect();

    //println!("{}", keys.len());

    for loc in keys.iter() {
      let alive: bool = self.get(&loc);
      let neighbors: [Loc;8] = neighbors(&loc);
      let alive_neighbors: usize = neighbors.iter().map(|neighbor| get(self.current_buffer(), neighbor)).filter(|alive| *alive).count();

      self.set(&loc, new_status(alive, alive_neighbors));
    }

    /*
    let keys: Vec<Loc> = self.next_buffer().keys().map(|&loc| loc).collect();
    for loc in keys.iter() {
      let alive: bool = get(self.next_buffer(), loc);
      let alive_neighbors: usize = neighbors(&loc).iter().map(|neighbor| get(self.next_buffer(), neighbor)).filter(|alive| *alive).count();

      if !alive && alive_neighbors == 0 {
        self.next_buffer().remove(&loc);
      }
    }*/

    self.using_buffer_1 = !self.using_buffer_1;
  }
}

fn get(buffer: &HashMap<Loc,bool>, loc: &Loc) -> bool {
  *buffer.get(loc).unwrap_or(&false)
}

fn new_status(alive: bool, alive_neighbors: usize) -> bool {
  if alive && (alive_neighbors == 2 || alive_neighbors == 3) {
    true
  } else if !alive && alive_neighbors == 3 {
    true
  } else {
    false
  }
}
