use crate::{bus::{self, Bus}, Database};


pub trait ShowBus {
  fn show_bus(&self, id: bus::Id) -> Option<&Bus>;
}

impl ShowBus for Database {
  fn show_bus(&self, id: bus::Id) -> Option<&Bus> {
    self.buses.iter().find(|b| b.id == id)
  }
}
