use crate::stop;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Id(pub u64);

#[derive(Debug)]
pub struct Route {
  pub id: Id,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StopId(pub u64);

#[derive(Debug, Default)]
pub struct Stop {
  pub id: StopId,
  #[allow(clippy::struct_field_names)]
  pub stop_id: stop::Id,
  pub route_id: Id,
  pub index: u8,
}
