use {
  crate::{
    bus::Bus,
    bus_model::BusModel,
    route::{self, Route},
    seat_reservation::{SeatReservation, SeatReservations},
    show_bus::ShowBus,
    show_bus_model::ShowBusModel,
    show_route::ShowRoute,
    show_trip::ShowTrip,
    trip::{self, Trip},
    Database,
  },
  std::{error, fmt::Display},
};

#[derive(Clone, Default)]
pub struct Form {
  pub seat_index: usize,
  pub from_stop_index: usize,
  pub to_stop_index: usize,
  pub trip_id: trip::Id,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
  TripNotFound,
  RouteNotFound,
  BusNotFound,
  BusModelNotFound,
  InvalidFromStop,
  InvalidToStop,
  InvalidSeat,
  SeatAlreadyReserved,
}

impl Display for Error {
  fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
impl error::Error for Error {}

pub struct Aggregate<'a> {
  route_stops: Vec<&'a route::Stop>,
  bus_model: &'a BusModel,
}

impl<'a> Aggregate<'a> {
  fn stops_count(&self) -> usize {
    self.route_stops.len()
  }

  const fn seats_count(&self) -> usize {
    self.bus_model.number_of_seats as usize
  }
}

pub trait ReserveSeat {
  fn reserve_seat(&mut self, form: Form) -> Result<(), Error>;

  fn build_aggregate(&self, trip_id: trip::Id) -> Result<Aggregate, Error>;
}

impl ReserveSeat for Database {
  fn build_aggregate(&self, trip_id: trip::Id) -> Result<Aggregate, Error> {
    let trip: &Trip = self.show_trip(trip_id).ok_or(Error::TripNotFound)?;
    let route: &Route = self.show_route(trip.route_id).ok_or(Error::RouteNotFound)?;
    let bus: &Bus = self.show_bus(trip.bus_id).ok_or(Error::BusNotFound)?;
    let bus_model: &BusModel = self
      .show_bus_model(bus.bus_model_id)
      .ok_or(Error::BusModelNotFound)?;
    let route_stops: Vec<&route::Stop> = self
      .route_stops
      .iter()
      .filter(|rs| rs.route_id == route.id)
      .collect();

    Ok(Aggregate {
      route_stops,
      bus_model,
    })
  }

  fn reserve_seat(&mut self, form: Form) -> Result<(), Error> {
    let aggr = self.build_aggregate(form.trip_id)?;

    if form.from_stop_index >= aggr.stops_count() {
      return Err(Error::InvalidFromStop);
    }

    if form.to_stop_index >= aggr.stops_count() {
      return Err(Error::InvalidToStop);
    }

    if form.seat_index >= aggr.seats_count() {
      return Err(Error::InvalidSeat);
    }

    let seat_reservation: SeatReservation = SeatReservation::new(
      form.trip_id,
      form.seat_index,
      form.from_stop_index,
      form.to_stop_index,
      aggr.seats_count(),
      aggr.stops_count(),
    );

    let seat_reservations: SeatReservations = self
      .seat_reservations
      .iter()
      .filter(|sr| sr.trip_id() == form.trip_id)
      .try_fold(
        SeatReservations::new(form.trip_id, aggr.seats_count(), aggr.stops_count()),
        |acc, e| acc.try_add(e),
      ).unwrap();

    if seat_reservations.matrix.has_colliding_bits(&seat_reservation.matrix) {
      return Err(Error::SeatAlreadyReserved);
    }

    self.seat_reservations.push(seat_reservation);

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use {
    super::*,
    crate::{bus, bus_model},
  };

  #[test]
  fn trip_not_found() {
    let mut database: Database = Database::default();
    let form = Form {
      seat_index: 0,
      from_stop_index: 0,
      to_stop_index: 1,
      trip_id: trip::Id(0),
    };
    let result = database.reserve_seat(form);

    assert_eq!(result, Err(Error::TripNotFound));
  }

  #[test]
  fn route_not_found() {
    let mut database: Database = Database {
      trips: vec![Trip {
        id: trip::Id(0),
        ..Default::default()
      }],
      ..Database::default()
    };
    let form = Form {
      seat_index: 0,
      from_stop_index: 0,
      to_stop_index: 1,
      trip_id: trip::Id(0),
    };
    let result = database.reserve_seat(form);

    assert_eq!(result, Err(Error::RouteNotFound));
  }

  #[test]
  fn bus_not_found() {
    let mut database: Database = Database {
      trips: vec![Trip {
        id: trip::Id(0),
        route_id: route::Id(0),
        ..Default::default()
      }],
      routes: vec![Route { id: route::Id(0) }],
      ..Database::default()
    };
    let form = Form {
      seat_index: 0,
      from_stop_index: 0,
      to_stop_index: 1,
      trip_id: trip::Id(0),
    };
    let result = database.reserve_seat(form);

    assert_eq!(result, Err(Error::BusNotFound));
  }

  #[test]
  fn bus_model_not_found() {
    let mut database: Database = Database {
      trips: vec![Trip {
        id: trip::Id(0),
        route_id: route::Id(0),
        ..Default::default()
      }],
      routes: vec![Route { id: route::Id(0) }],
      buses: vec![Bus {
        id: bus::Id(0),
        bus_model_id: bus_model::Id(0),
      }],
      ..Database::default()
    };

    let form = Form {
      seat_index: 0,
      from_stop_index: 0,
      to_stop_index: 1,
      trip_id: trip::Id(0),
    };

    let result = database.reserve_seat(form);

    assert_eq!(result, Err(Error::BusModelNotFound));
  }

  #[test]
  fn invalid_from_stop() {
    let mut database: Database = Database {
      trips: vec![Trip {
        id: trip::Id(0),
        route_id: route::Id(0),
        ..Default::default()
      }],
      routes: vec![Route { id: route::Id(0) }],
      buses: vec![Bus {
        id: bus::Id(0),
        bus_model_id: bus_model::Id(0),
      }],
      bus_models: vec![BusModel {
        id: bus_model::Id(0),
        ..Default::default()
      }],
      ..Database::default()
    };

    let form = Form {
      seat_index: 0,
      from_stop_index: 0,
      to_stop_index: 1,
      trip_id: trip::Id(0),
    };
    let result = database.reserve_seat(form);

    assert_eq!(result, Err(Error::InvalidFromStop));
  }

  #[test]
  fn invalid_to_stop() {
    let mut database: Database = Database {
      trips: vec![Trip {
        id: trip::Id(0),
        route_id: route::Id(0),
        ..Default::default()
      }],
      routes: vec![Route { id: route::Id(0) }],
      route_stops: vec![route::Stop {
        route_id: route::Id(0),
        ..Default::default()
      }],
      buses: vec![Bus {
        id: bus::Id(0),
        bus_model_id: bus_model::Id(0),
      }],
      bus_models: vec![BusModel {
        id: bus_model::Id(0),
        ..Default::default()
      }],
      ..Database::default()
    };

    let form = Form {
      seat_index: 0,
      from_stop_index: 0,
      to_stop_index: 1,
      trip_id: trip::Id(0),
    };
    let result = database.reserve_seat(form);

    assert_eq!(result, Err(Error::InvalidToStop));
  }

  #[test]
  fn invalid_seat() {
    let mut database: Database = Database {
      trips: vec![Trip {
        id: trip::Id(0),
        route_id: route::Id(0),
        ..Default::default()
      }],
      routes: vec![Route { id: route::Id(0) }],
      route_stops: vec![
        route::Stop {
          route_id: route::Id(0),
          ..Default::default()
        },
        route::Stop {
          route_id: route::Id(0),
          ..Default::default()
        },
      ],
      buses: vec![Bus {
        id: bus::Id(0),
        bus_model_id: bus_model::Id(0),
      }],
      bus_models: vec![BusModel {
        id: bus_model::Id(0),
        ..Default::default()
      }],
      ..Database::default()
    };

    let form = Form {
      seat_index: 0,
      from_stop_index: 0,
      to_stop_index: 1,
      trip_id: trip::Id(0),
    };
    let result = database.reserve_seat(form);

    assert_eq!(result, Err(Error::InvalidSeat));
  }

  #[test]
  fn ok_test() {
    let mut database: Database = Database {
      trips: vec![Trip {
        id: trip::Id(0),
        route_id: route::Id(0),
        ..Default::default()
      }],
      routes: vec![Route { id: route::Id(0) }],
      route_stops: vec![
        route::Stop {
          route_id: route::Id(0),
          ..Default::default()
        },
        route::Stop {
          route_id: route::Id(0),
          ..Default::default()
        },
      ],
      buses: vec![Bus {
        id: bus::Id(0),
        bus_model_id: bus_model::Id(0),
      }],
      bus_models: vec![BusModel {
        id: bus_model::Id(0),
        number_of_seats: 1,
        ..Default::default()
      }],
      ..Database::default()
    };

    let form = Form {
      seat_index: 0,
      from_stop_index: 0,
      to_stop_index: 1,
      trip_id: trip::Id(0),
    };

    let result = database.reserve_seat(form);

    assert_eq!(result, Ok(()));
  }

  #[test]
  fn already_reserved() {
    let mut database: Database = Database {
      trips: vec![Trip {
        id: trip::Id(0),
        route_id: route::Id(0),
        ..Default::default()
      }],
      routes: vec![Route { id: route::Id(0) }],
      route_stops: vec![
        route::Stop {
          route_id: route::Id(0),
          ..Default::default()
        },
        route::Stop {
          route_id: route::Id(0),
          ..Default::default()
        },
      ],
      buses: vec![Bus {
        id: bus::Id(0),
        bus_model_id: bus_model::Id(0),
      }],
      bus_models: vec![BusModel {
        id: bus_model::Id(0),
        number_of_seats: 1,
        ..Default::default()
      }],
      ..Database::default()
    };

    let form = Form {
      seat_index: 0,
      from_stop_index: 0,
      to_stop_index: 1,
      trip_id: trip::Id(0),
    };

    let result = database.reserve_seat(form.clone());
    let second_result = database.reserve_seat(form);

    assert_eq!(result, Ok(()));
    assert_eq!(second_result, Err(Error::SeatAlreadyReserved));
  }
}
