use std::ops::Range;

use crate::{SeatReservation, Ticket, Trip, SEAT_RESERVATIONS, TICKETS, TRIPS};

#[derive(Debug, Clone)]
pub struct SeatMap {
  pub seat_count: usize,
  pub stop_count: usize,
  pub available: Vec<bool>,
  pub bought: Vec<bool>,
  pub reserved: Vec<bool>,
  pub unavailable: Vec<bool>,
}

pub fn build_seat_map(trip_id: usize) -> SeatMap {
  let trip: Trip = TRIPS.lock().unwrap().find(trip_id).unwrap();
  let seat_count: usize = trip.seat_count;
  let stop_count: usize = trip.stop_count;
  let mut available: Vec<bool> = vec![true; seat_count * stop_count];
  let mut bought: Vec<bool> = vec![false; seat_count * stop_count];
  let mut reserved: Vec<bool> = vec![false; seat_count * stop_count];
  let mut unavailable: Vec<bool> = vec![false; seat_count * stop_count];
  let tickets: Vec<Ticket> = TICKETS
    .lock()
    .unwrap()
    .records
    .iter()
    .filter(|ticket| ticket.trip_id == trip.id)
    .cloned()
    .collect();

  let seat_reservations: Vec<SeatReservation> = SEAT_RESERVATIONS
    .lock()
    .unwrap()
    .records
    .iter()
    .filter(|trip_sale| trip_sale.trip_id == trip.id)
    .cloned()
    .collect();

  tickets.iter().for_each(|ticket| {
    let range: Range<usize> = (ticket.seat * trip.stop_count + ticket.from_stop())
      ..(ticket.seat * trip.stop_count + ticket.to_stop());

    for i in range {
      available[i] = false;
      bought[i] = true;
      unavailable[i] = true;
    }
  });

  seat_reservations.iter().for_each(|seat_reservation| {
    seat_reservation.reserved().iter().enumerate().for_each(
      |(i, is_reserved)| match *is_reserved {
        true => {
          available[i] = false;
          reserved[i] = true;
          unavailable[i] = true;
        }
        false => (),
      },
    );
  });

  SeatMap { seat_count, stop_count, available, bought, reserved, unavailable }
}

impl SeatReservation {
  pub fn reserved(&self) -> Vec<bool> {
    let mut reserved: Vec<bool> = vec![false; self.seat_count * self.stop_count];
    let range: Range<usize> = (self.seat * self.stop_count + self.from_stop())
      ..(self.seat * self.stop_count + self.to_stop());
    for i in range {
      reserved[i] = true;
    }
    reserved
  }
}
