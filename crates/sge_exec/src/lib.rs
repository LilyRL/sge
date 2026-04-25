use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

pub use futures;
use sge_time::time;

pub mod fs;
#[cfg(feature = "network")]
pub mod net;

fn noop_waker() -> Waker {
    unsafe fn clone(data: *const ()) -> RawWaker {
        RawWaker::new(data, &VTABLE)
    }
    unsafe fn wake(_: *const ()) {}
    unsafe fn wake_by_ref(_: *const ()) {}
    unsafe fn drop(_: *const ()) {}

    const VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
    unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) }
}

pub fn poll_once<T>(future: &mut Pin<Box<dyn Future<Output = T>>>) -> Option<T> {
    let waker = noop_waker();
    let mut cx = Context::from_waker(&waker);
    match future.as_mut().poll(&mut cx) {
        Poll::Ready(v) => Some(v),
        Poll::Pending => None,
    }
}

#[derive(Clone)]
pub struct Coroutine {
    done: Arc<Mutex<bool>>,
}

impl Coroutine {
    pub fn is_done(&self) -> bool {
        *self.done.lock().unwrap()
    }
}

struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
    done: Arc<Mutex<bool>>,
}

#[derive(Default)]
pub struct Executor {
    tasks: Vec<Task>,
}

impl Executor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn spawn(&mut self, future: impl Future<Output = ()> + 'static) -> Coroutine {
        let done = Arc::new(Mutex::new(false));
        self.tasks.push(Task {
            future: Box::pin(future),
            done: done.clone(),
        });

        Coroutine { done }
    }

    pub fn tick(&mut self) {
        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);

        self.tasks
            .retain_mut(|task| match task.future.as_mut().poll(&mut cx) {
                Poll::Ready(()) => {
                    *task.done.lock().unwrap() = true;
                    false
                }
                Poll::Pending => true,
            });
    }
}

sge_global::global!(Executor, executor);

#[derive(Default)]
pub struct FrameFuture {
    done: bool,
}

impl Future for FrameFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.done {
            Poll::Ready(())
        } else {
            self.done = true;
            Poll::Pending
        }
    }
}

pub fn start_coroutine(future: impl Future<Output = ()> + 'static) -> Coroutine {
    get_executor().spawn(future)
}

pub fn init() {
    set_executor(Executor::new());
}

pub struct WaitFuture {
    time: f32,
}

impl Future for WaitFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        if time() < self.time {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

pub fn wait_for(seconds: f32) -> WaitFuture {
    WaitFuture {
        time: time() + seconds,
    }
}

pub struct WaitFramesFuture {
    frames: usize,
}

impl Future for WaitFramesFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        if self.frames == 0 {
            Poll::Ready(())
        } else {
            self.frames -= 1;
            Poll::Pending
        }
    }
}

pub fn wait_for_frames(frames: usize) -> WaitFramesFuture {
    WaitFramesFuture { frames }
}
