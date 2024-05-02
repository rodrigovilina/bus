use crate::{stop, Database};


pub trait CreateStop {
  fn create_stop(&mut self, form: stop::Stop);
}

impl CreateStop for Database {
  fn create_stop(&mut self, form: stop::Stop) {
    self.stops.push(form);
  }
}
