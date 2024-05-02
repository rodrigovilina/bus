#![warn(clippy::complexity)]
#![warn(clippy::expect_used)]
#![warn(clippy::nursery)]
#![warn(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![warn(clippy::unwrap_used)]

mod bus;
mod bus_model;
mod create_bus;
mod create_bus_model;
mod create_route;
mod create_stop;
mod create_trip;
mod reserve_seat;
mod route;
mod show_bus;
mod show_bus_model;
mod show_stop;
mod stop;
mod trip;
mod show_trip;
mod show_route;
mod seat_reservation;
mod bool_matrix;

use seat_reservation::SeatReservation;

use {reserve_seat::ReserveSeat, trip::Trip};

use crate::{
  bus::Bus,
  bus_model::BusModel,
  create_bus::CreateBus,
  create_bus_model::CreateBusModel,
  create_route::CreateRoute,
  create_stop::CreateStop,
  create_trip::CreateTrip,
  route::{Route, StopId},
  show_bus::ShowBus,
  show_bus_model::ShowBusModel,
  show_stop::ShowStop,
};

#[derive(Debug, Default)]
struct Database {
  bus_models: Vec<BusModel>,
  buses: Vec<Bus>,
  stops: Vec<stop::Stop>,
  route_stops: Vec<route::Stop>,
  routes: Vec<Route>,
  trips: Vec<Trip>,
  seat_reservations: Vec<SeatReservation>
}

fn main() {
  let mut database: Database = Database {
    bus_models: vec![],
    buses: vec![],
    stops: vec![],
    route_stops: vec![],
    routes: vec![],
    trips: vec![],
    seat_reservations: vec![],
  };
  database.create_bus_model(BusModel {
    id: bus_model::Id(0),
    name: "Modelo 1".to_string(),
    number_of_seats: 1,
  });
  database.create_bus(Bus {
    id: bus::Id(0),
    bus_model_id: bus_model::Id(0),
  });
  database.create_stop(stop::Stop {
    id: stop::Id(0),
    name: "Stop 0".to_string(),
  });
  database.create_stop(stop::Stop {
    id: stop::Id(1),
    name: "Stop 1".to_string(),
  });
  database.create_route(create_route::Form {
    route: Route { id: route::Id(0) },
    stops: vec![
      route::Stop {
        id: StopId(0),
        index: 0,
        stop_id: stop::Id(0),
        route_id: route::Id(0),
      },
      route::Stop {
        id: StopId(1),
        index: 1,
        stop_id: stop::Id(1),
        route_id: route::Id(0),
      },
    ],
  });
  database.create_trip(create_trip::Form {
    trip: Trip {
      id: trip::Id(0),
      route_id: route::Id(0),
      bus_id: bus::Id(0),
    },
  });
  database.show_bus_model(bus_model::Id(0));
  database.show_bus(bus::Id(0));
  database.show_stop(stop::Id(0));
  database.show_stop(stop::Id(1));

  let _ = database.reserve_seat(reserve_seat::Form {
    seat_index: 0,
    from_stop_index: 0,
    to_stop_index: 0,
    trip_id: trip::Id(1),
  });
}

