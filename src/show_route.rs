use crate::{
  route::{self, Route},
  Database,
};

pub trait ShowRoute {
  fn show_route(&self, id: route::Id) -> Option<&Route>;
}

impl ShowRoute for Database {
  fn show_route(&self, id: route::Id) -> Option<&Route> {
    self.routes.iter().find(|r| r.id == id)
  }
}
