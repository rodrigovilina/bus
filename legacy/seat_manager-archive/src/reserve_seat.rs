use std::ops::Range;

use crate::{
  build_seat_map::{build_seat_map, SeatMap},
  repo::MutexRepo,
  SeatReservation, Trip, SEAT_RESERVATIONS, TRIPS,
};

pub fn reserve_seat(
  seat: usize,
  stops: (usize, usize),
  trip_id: usize,
  user_id: usize,
) -> Result<(), String> {
  let trip: Trip = TRIPS.lock().unwrap().find(trip_id).unwrap();
  let seat_map: SeatMap = build_seat_map(trip_id);
  let seat_count: usize = trip.seat_count;
  let stop_count: usize = trip.stop_count;
  let reservation: Vec<bool> = seat_request(seat_count, stop_count, seat, stops);

  check_availabilty(reservation, seat_map)?;

  SEAT_RESERVATIONS.create(SeatReservation {
    id: 0,
    trip_id,
    user_id,
    seat,
    seat_count,
    stop_count,
    stops,
  })
}

fn check_availabilty(reservation: Vec<bool>, seat_map: SeatMap) -> Result<(), String> {
  for (is_reserved, is_available) in reservation.iter().zip(seat_map.available.iter()) {
    if let (true, false) = (is_reserved, is_available) {
      return Err(String::from("Seat is not available"));
    }
  }
  Ok(())
}

fn seat_request(
  seat_count: usize,
  stop_count: usize,
  seat: usize,
  stops: (usize, usize),
) -> Vec<bool> {
  let mut seat_request: Vec<bool> = vec![false; seat_count * stop_count];

  let range: Range<usize> = (seat * stop_count + stops.0)..(seat * stop_count + stops.1);
  for i in range {
    seat_request[i] = true;
  }
  seat_request
}
