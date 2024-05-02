#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Id(pub u64);

#[derive(Debug, Default, PartialEq, Eq)]
pub struct BusModel {
  pub id: Id,
  pub name: String,
  pub number_of_seats: u8,
}
