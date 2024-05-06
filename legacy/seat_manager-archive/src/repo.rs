use std::sync::Mutex;

pub struct Repo<A> {
  pub records: Vec<A>,
}

pub trait HasID {
  fn id(&self) -> usize;

  fn with_id(self, id: usize) -> Self;
}

impl<A> Repo<A>
where
  A: Clone + HasID,
{
  pub const fn new() -> Repo<A> { Repo { records: Vec::new() } }

  pub fn first(&self) -> Option<A> { self.records.first().cloned() }

  pub fn push(&mut self, record: A) { self.records.push(record); }

  pub fn find(&self, id: usize) -> Option<A> {
    self.records.iter().find(|record| record.id() == id).cloned()
  }

  pub fn last(&self) -> Option<A> { self.records.last().cloned() }

  pub fn next_id(&self) -> usize {
    self.records.iter().map(|record| record.id()).max().map(|id| id + 1).unwrap_or(1)
  }

  pub fn clear(&mut self) { self.records.clear() }
}

pub trait MutexRepo<A> {
  fn first(&self) -> Option<A>;

  fn last(&self) -> Option<A>;

  fn next_id(&self) -> usize;

  fn clear(&self);

  fn create(&self, record: A) -> Result<(), String>;
}

impl<A> MutexRepo<A> for Mutex<Repo<A>>
where
  A: Clone + HasID,
{
  fn first(&self) -> Option<A> { self.lock().unwrap().first() }

  fn last(&self) -> Option<A> { self.lock().unwrap().last() }

  fn next_id(&self) -> usize { self.lock().unwrap().next_id() }

  fn clear(&self) { self.lock().unwrap().clear() }

  fn create(&self, record: A) -> Result<(), String> {
    let record: A = record.with_id(self.next_id());
    self.lock().unwrap().push(record);
    Ok(())
  }
}
