use crate::{bus, route};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Id(pub u64);

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Trip {
  pub id: Id,
  pub route_id: route::Id,
  pub bus_id: bus::Id,
}
