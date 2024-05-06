use crate::repo::HasID;

#[derive(Debug, Clone)]
pub struct SeatReservation {
  pub id: usize,
  pub seat_count: usize,
  pub stop_count: usize,
  pub seat: usize,
  pub stops: (usize, usize),
  pub trip_id: usize,
  pub user_id: usize,
}

impl SeatReservation {
  pub fn from_stop(&self) -> usize { self.stops.0 }

  pub fn to_stop(&self) -> usize { self.stops.1 }
}

impl HasID for SeatReservation {
  fn id(&self) -> usize { self.id }

  fn with_id(self, id: usize) -> Self { SeatReservation { id, ..self } }
}
