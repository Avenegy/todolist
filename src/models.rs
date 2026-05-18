use std::cell::{Cell};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
       pub name: String,
       pub done: bool,
    }


pub struct IDManager {
  next_id: Cell<i64>
}

impl IDManager {
  pub fn new(start: i64) -> IDManager {
    IDManager { next_id: Cell::new(start) }
  }

  pub fn get_id(&self) -> i64 {
    let ans = self.next_id.get(); 
    self.next_id.set(ans + 1); 
    ans
  }
}
