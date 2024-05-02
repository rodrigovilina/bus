use crate::{trip::Trip, Database};


pub struct Form {
  pub trip: Trip,
}

pub trait CreateTrip {
  fn create_trip(&mut self, form: Form);
}

impl CreateTrip for Database {
  fn create_trip(&mut self, form: Form) {
    self.trips.push(form.trip);
  }
}
