use crate::{
  stop,
  Database,
};

pub trait ShowStop {
  fn show_stop(&self, id: stop::Id) -> Option<&stop::Stop>;
}

impl ShowStop for Database {
  fn show_stop(&self, id: stop::Id) -> Option<&stop::Stop> {
    self.stops.iter().find(|s| s.id == id)
  }
}
