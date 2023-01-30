use futures::task::waker_ref;
use std::sync::Arc;
use std::task::Context;

mod symmetry_coroutine;

fn main() {
    do_symmetry_coroutine();
}

fn do_symmetry_coroutine() {
    let task = Arc::new(symmetry_coroutine::Task::new());
    let waker = waker_ref(&task);
    let mut context = Context::from_waker(&waker);
    let mut hello = task.hello.lock().unwrap();

    hello.as_mut().poll(&mut context);
    hello.as_mut().poll(&mut context);
    hello.as_mut().poll(&mut context);
}
