use crate::repo::HasID;

#[derive(Debug, Clone)]
pub struct Ticket {
  pub id: usize,
  pub trip_id: usize,
  pub user_id: usize,
  pub seat: usize,
  pub stops: (usize, usize),
}

impl Ticket {
  pub fn from_stop(&self) -> usize { self.stops.0 }

  pub fn to_stop(&self) -> usize { self.stops.1 }
}

impl HasID for Ticket {
  fn id(&self) -> usize { self.id }

  fn with_id(self, id: usize) -> Self { Ticket { id, ..self } }
}
