use chrono::Duration as ChronoDuration;
use parking_lot::{Condvar, Mutex, RwLock};
use std::{
	cmp::Ordering,
	collections::BinaryHeap,
	fmt::{Debug, Formatter, Result as FmtResult},
	sync::Arc,
	time::Duration as StdDuration,
};
use supernova::cloned;
use threadpool::ThreadPool;

pub use chrono::{DateTime, Duration, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Context {
	time: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DateResult {
	Done,
	Repeat(DateTime<Utc>),
}

pub struct Date {
	pub context: Context,
	pub job: Box<dyn FnMut(&mut Context) -> DateResult + Send + Sync + 'static>,
}

impl Debug for Date {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.debug_struct("Date")
			.field("context", &self.context)
			.field("job", &"job fn")
			.finish()
	}
}

impl PartialEq for Date {
	fn eq(&self, other: &Self) -> bool {
		self.context.time == other.context.time
	}
}

impl Eq for Date {}

impl PartialOrd for Date {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.context.time.cmp(&other.context.time).reverse())
	}
}

impl Ord for Date {
	fn cmp(&self, other: &Self) -> Ordering {
		self.context.time.cmp(&other.context.time).reverse()
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum SchedulerState {
	PauseEmpty,
	PauseTime(StdDuration),
	Run,
	Exit,
}

impl SchedulerState {
	fn is_running(self) -> bool {
		matches!(self, SchedulerState::Run)
	}

	fn new_pause_time(duration: ChronoDuration) -> Self {
		Self::PauseTime(
			duration
				.to_std()
				.unwrap_or_else(|_| StdDuration::from_millis(0)),
		)
	}
}

impl Default for SchedulerState {
	fn default() -> Self {
		Self::PauseEmpty
	}
}

#[derive(Debug, Clone)]
pub struct Scheduler {
	condvar: Arc<(Mutex<SchedulerState>, Condvar)>,
	dates: Arc<RwLock<BinaryHeap<Date>>>,
}

impl Scheduler {
	pub fn new(thread_count: usize) -> Self {
		let pair: Arc<(Mutex<SchedulerState>, Condvar)> = Arc::default();
		let pair_scheduler = pair.clone();
		let dates: Arc<RwLock<BinaryHeap<Date>>> = Arc::default();

		cloned!(dates => std::thread::spawn(move || {
			let &(ref state_lock, ref notifier) = &*pair_scheduler;
			let threadpool = ThreadPool::new(thread_count);

			loop {
				if let Break::Yes = process_states(&state_lock, &notifier) {
					break;
				}

				dispatch_date(&threadpool, &dates, &pair_scheduler);

				check_peeking_date(&dates, &state_lock);
			}
		}));

		Self {
			condvar: pair,
			dates,
		}
	}

	pub fn add_task_datetime<T>(&mut self, time: DateTime<Utc>, to_execute: T)
	where
		T: FnMut(&mut Context) -> DateResult + Send + Sync + 'static,
	{
		let &(ref state_lock, ref notifier) = &*self.condvar;

		let task = Date {
			context: Context { time },
			job: Box::new(to_execute),
		};

		let mut locked_heap = self.dates.write();

		if locked_heap.is_empty() {
			let mut scheduler_state = state_lock.lock();
			let left = task.context.time.signed_duration_since(Utc::now());

			if !scheduler_state.is_running() {
				*scheduler_state = SchedulerState::new_pause_time(left);
				notifier.notify_one();
			}
		} else {
			let mut scheduler_state = state_lock.lock();

			if let SchedulerState::PauseTime(_) = *scheduler_state {
				let peeked = locked_heap.peek().expect("expected heap to be filled");

				if task.context.time < peeked.context.time {
					let left = task.context.time.signed_duration_since(Utc::now());

					if !scheduler_state.is_running() {
						*scheduler_state = SchedulerState::PauseTime(
							left.to_std()
								.unwrap_or_else(|_| StdDuration::from_millis(0)),
						);
						notifier.notify_one();
					}
				}
			}
		}

		locked_heap.push(task);
	}

	pub fn add_task_duration<T>(&mut self, how_long: ChronoDuration, to_execute: T)
	where
		T: FnMut(&mut Context) -> DateResult + Send + Sync + 'static,
	{
		let time = Utc::now() + how_long;
		self.add_task_datetime(time, to_execute);
	}
}

impl<'a> Drop for Scheduler {
	fn drop(&mut self) {
		let &(ref state_lock, ref notifier) = &*self.condvar;

		let mut state = state_lock.lock();
		*state = SchedulerState::Exit;
		notifier.notify_one();
	}
}

fn set_state_lock(state_lock: &Mutex<SchedulerState>, to_set: SchedulerState) {
	let mut state = state_lock.lock();
	*state = to_set;
}

#[inline]
fn _push_and_notify(date: Date, heap: &mut BinaryHeap<Date>, notifier: &Condvar) {
	heap.push(date);
	notifier.notify_one();
}

#[inline]
fn push_and_notify(
	dispatcher_pair: &Arc<(Mutex<SchedulerState>, Condvar)>,
	data_pooled: &Arc<RwLock<BinaryHeap<Date>>>,
	when: &DateTime<Utc>,
	date: Date,
) {
	let &(ref state_lock, ref notifier) = &**dispatcher_pair;

	let mut state = state_lock.lock();

	let mut heap_lock = data_pooled.write();

	if let Some(peek) = heap_lock.peek() {
		if peek.context.time < *when {
			let left = peek.context.time.signed_duration_since(Utc::now());

			*state = SchedulerState::new_pause_time(left);
			_push_and_notify(date, &mut heap_lock, &notifier);
		} else {
			let left = when.signed_duration_since(Utc::now());

			*state = SchedulerState::new_pause_time(left);
			_push_and_notify(date, &mut heap_lock, &notifier);
		}
	} else {
		let left = when.signed_duration_since(Utc::now());

		*state = SchedulerState::new_pause_time(left);
		_push_and_notify(date, &mut heap_lock, &notifier);
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Break {
	Yes,
	No,
}

#[inline]
fn process_states(state_lock: &Mutex<SchedulerState>, notifier: &Condvar) -> Break {
	let mut scheduler_state = state_lock.lock();

	while let SchedulerState::PauseEmpty = *scheduler_state {
		notifier.wait(&mut scheduler_state);
	}

	while let SchedulerState::PauseTime(duration) = *scheduler_state {
		if notifier
			.wait_for(&mut scheduler_state, duration)
			.timed_out()
		{
			break;
		}
	}

	if let SchedulerState::Exit = *scheduler_state {
		Break::Yes
	} else {
		Break::No
	}
}

fn dispatch_date(
	threadpool: &ThreadPool,
	dates: &Arc<RwLock<BinaryHeap<Date>>>,
	pair_scheduler: &Arc<(Mutex<SchedulerState>, Condvar)>,
) {
	let mut date = {
		let mut dates = dates.write();

		dates.pop().expect("Should not run on empty heap.")
	};

	let date_dispatcher = dates.clone();
	let dispatcher_pair = pair_scheduler.clone();

	threadpool.execute(move || {
		if let DateResult::Repeat(when) = (date.job)(&mut date.context) {
			date.context.time = when;

			push_and_notify(&dispatcher_pair, &date_dispatcher, &when, date);
		}
	});
}

fn check_peeking_date(dates: &Arc<RwLock<BinaryHeap<Date>>>, state_lock: &Mutex<SchedulerState>) {
	if let Some(next) = dates.read().peek() {
		let now = Utc::now();

		if next.context.time > now {
			let left = next.context.time.signed_duration_since(now);

			set_state_lock(&state_lock, SchedulerState::new_pause_time(left));
		} else {
			set_state_lock(&state_lock, SchedulerState::Run);
		}
	} else {
		set_state_lock(&state_lock, SchedulerState::PauseEmpty);
	}
}
