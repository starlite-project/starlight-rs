use clokwerk::Scheduler;
use nebula::Leak;
use std::{fmt::{Debug, Formatter, Result as FmtResult}, sync::Arc, time::Duration};
use tokio::{sync::Mutex, task::JoinHandle, time::interval};

#[derive(Default)]
pub struct Schedule(ScheduleComponents);

impl Schedule {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn start(&mut self, frequency: Duration) {
        let mut timer = interval(frequency);

        let mut lock = self.0.schedule.clone().lock_owned().await;

        let handle = tokio::spawn(async move {
            lock.run_pending();
            timer.tick().await;
        });

        self.0.handle = Arc::new(Some(handle));
    }
}

impl Debug for Schedule {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.debug_tuple("Schedule").field(&"scheduler").finish()
	}
}

#[derive(Default)]
struct ScheduleComponents {
	schedule: Arc<Mutex<Scheduler>>,
	handle: Arc<Option<JoinHandle<()>>>,
}