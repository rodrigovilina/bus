use crate::{bus_model::BusModel, Database};


pub trait CreateBusModel {
  fn create_bus_model(&mut self, form: BusModel);
}

impl CreateBusModel for Database {
  fn create_bus_model(&mut self, form: BusModel) {
    self.bus_models.push(form);
  }
}
