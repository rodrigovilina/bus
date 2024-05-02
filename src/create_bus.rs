use crate::{bus::Bus, Database};


pub trait CreateBus {
  fn create_bus(&mut self, form: Bus);
}

impl CreateBus for Database {
  fn create_bus(&mut self, form: Bus) {
    self.buses.push(form);
  }
}
