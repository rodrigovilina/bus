use crate::{trip::{self, Trip}, Database};

pub trait ShowTrip {
  fn show_trip(&self, id: trip::Id) -> Option<&Trip>;
}

impl ShowTrip for Database {
  fn show_trip(&self, id: trip::Id) -> Option<&Trip> {
    self.trips.iter().find(|t| t.id == id)
  }
}
