use crate::{
  repo::MutexRepo, seat_reservation::SeatReservation, ticket::Ticket, SEAT_RESERVATIONS, TICKETS,
};

type Stops = (usize, usize);

pub fn create_ticket(
  seat: usize,
  stops: Stops,
  trip_id: usize,
  user_id: usize,
) -> Result<(), String> {
  let ticket: Ticket = Ticket { id: 0, trip_id, seat, stops, user_id };

  let seat_reservation: Option<SeatReservation> = SEAT_RESERVATIONS
    .lock()
    .unwrap()
    .records
    .iter()
    .find(|sr| {
      sr.trip_id == trip_id
        && sr.seat == seat
        && sr.from_stop() == stops.0
        && sr.to_stop() == stops.1
    })
    .cloned();

  match seat_reservation {
    Some(_) => TICKETS.create(ticket)?,
    None => {
      return Err(format!(
        "No seat reservation found for trip {}, seat {}, from_stop {}, to_stop {}",
        trip_id, seat, stops.0, stops.1
      ))
    }
  }

  Ok(())
}
