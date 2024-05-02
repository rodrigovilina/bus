use crate::{
  bus_model::{self, BusModel},
  Database,
};

pub trait ShowBusModel {
  fn show_bus_model(&self, id: bus_model::Id) -> Option<&BusModel>;
}

impl ShowBusModel for Database {
  fn show_bus_model(&self, id: bus_model::Id) -> Option<&BusModel> {
    self.bus_models.iter().find(|bm| bm.id == id)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn none_test() {
    let database: Database = Database::default();

    assert_eq!(database.show_bus_model(bus_model::Id(0)), None);
  }

  #[test]
  fn some_test() {
    let bus_model: BusModel = BusModel {
      id: bus_model::Id(0),
      name: String::default(),
      number_of_seats: 1,
    };
    let database: Database = Database {
      bus_models: vec![bus_model],
      ..Default::default()
    };
    assert_eq!(database.show_bus_model(bus_model::Id(0)), Some(BusModel { id: bus_model::Id(0), name: String::default(), number_of_seats: 1 }).as_ref());
  }
}
