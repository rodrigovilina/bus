use crate::bus_model;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Id(pub u64);

#[derive(Debug, Default)]
pub struct Bus {
  pub id: Id,
  pub bus_model_id: bus_model::Id,
}
