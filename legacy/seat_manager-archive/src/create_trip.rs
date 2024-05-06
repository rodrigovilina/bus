use crate::{repo::MutexRepo, Trip, TRIPS};

pub fn create_trip(seat_count: usize, stop_count: usize) {
  TRIPS.create(Trip { id: 0, seat_count, stop_count }).unwrap();
}

// tests
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create_trip() {
    let seat_count: usize = 2;
    let stop_count: usize = 3;
    create_trip(seat_count, stop_count);
    let trip: Trip = TRIPS.lock().unwrap().first().unwrap();
    assert_eq!(trip.id, 1);
    assert_eq!(trip.seat_count, 2);
    assert_eq!(trip.stop_count, 3);
    TRIPS.clear();
  }

  #[test]
  fn test_create_two_trips() {
    let seat_count: usize = 2;
    let stop_count: usize = 3;
    create_trip(seat_count, stop_count);
    create_trip(seat_count, stop_count);
    let trip: Trip = TRIPS.last().unwrap();
    assert_eq!(trip.id, 2);
    assert_eq!(trip.seat_count, 2);
    assert_eq!(trip.stop_count, 3);
  }
}
