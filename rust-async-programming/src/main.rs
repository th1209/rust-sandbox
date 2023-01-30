use futures::task::waker_ref;
use std::sync::Arc;
use std::task::Context;

mod asymmetry_coroutine;
mod symmetry_coroutine;

fn main() {
    println!("do symmetry coroutine");
    do_symmetry_coroutine();

    println!("do asymmetry coroutine");
    do_asymmetry_coroutine();
}

fn do_symmetry_coroutine() {
    let task = Arc::new(symmetry_coroutine::Task::new());
    let waker = waker_ref(&task);
    let mut context = Context::from_waker(&waker);
    let mut hello = task.hello.lock().unwrap();

    let _ = hello.as_mut().poll(&mut context);
    _ = hello.as_mut().poll(&mut context);
    _ = hello.as_mut().poll(&mut context);
}

fn do_asymmetry_coroutine() {
    let executor = asymmetry_coroutine::Executor::new();
    executor
        .get_spawner()
        .spawn(asymmetry_coroutine::Hello::new());
    executor.run();
}
