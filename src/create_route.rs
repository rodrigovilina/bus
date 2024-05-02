use crate::{route::{self, Route}, Database};


pub struct Form {
  pub stops: Vec<route::Stop>,
  pub route: Route,
}

pub trait CreateRoute {
  fn create_route(&mut self, form: Form);
}

impl CreateRoute for Database {
  fn create_route(&mut self, form: Form) {
    for stop in form.stops {
      self.route_stops.push(stop);
    }
    self.routes.push(form.route);
  }
}
