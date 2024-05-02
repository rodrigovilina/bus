use crate::{bool_matrix::BoolMatrix, trip};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeatReservation {
  trip_id: trip::Id,
  seat: usize,
  from: usize,
  to: usize,
  pub matrix: BoolMatrix,
}

pub struct SeatReservations {
  trip_id: trip::Id,
  pub matrix: BoolMatrix,
}

impl SeatReservations {
  pub fn new(trip_id: trip::Id, seats: usize, stops: usize) -> Self {
    let matrix = BoolMatrix::new(stops, seats);

    Self { trip_id, matrix }
  }
  pub fn try_add(&self, reservation: &SeatReservation) -> Option<Self> {
    let matrix: BoolMatrix = BoolMatrix::try_add(&self.matrix, &reservation.matrix)?;

    Some(Self {
      trip_id: self.trip_id,
      matrix,
    })
  }
}

impl SeatReservation {
  pub const fn trip_id(&self) -> trip::Id {
    self.trip_id
  }

  pub fn new(
    trip_id: trip::Id,
    seat: usize,
    from: usize,
    to: usize,
    seats: usize,
    stops: usize,
  ) -> Self {
    let mut matrix = BoolMatrix::new(stops, seats);
    matrix.set_row_range(seat, from, to, true);
    Self {
      trip_id,
      seat,
      from,
      to,
      matrix,
    }
  }

  pub const fn seats(&self) -> usize {
    self.matrix.height
  }

  pub const fn stops(&self) -> usize {
    self.matrix.width
  }
}
