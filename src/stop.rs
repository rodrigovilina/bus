#[derive(Debug, Default, PartialEq, Eq)]
pub struct Id(pub u64);

#[derive(Debug)]
pub struct Stop {
  pub id: Id,
  pub name: String,
}
