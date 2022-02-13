mod indexed_local_dir;
mod local_dir;

use super::event::{Event, EventId};
use crate::util::Result;

use chrono::{DateTime, Local};

pub use indexed_local_dir::IndexedLocalDir;
pub use local_dir::{LocalDir, LocalDirBuilder};

pub trait Backend {
  fn get_event(&self, event_id: &EventId) -> Result<Event>;

  // get events which overlap with the from..to interval.
  fn get_events(
    &self,
    from: DateTime<Local>,
    to: DateTime<Local>,
  ) -> Result<Vec<Event>>;

  fn delete_event(&mut self, event_id: &EventId) -> Result<()>;

  fn update_event(&mut self, updated_event: &Event) -> Result<()>;

  fn create_event(&mut self, event: &Event) -> Result<()>;
}
