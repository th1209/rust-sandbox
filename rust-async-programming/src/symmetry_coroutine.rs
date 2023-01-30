use futures::future::{BoxFuture, FutureExt};
use futures::task::{waker_ref, ArcWake};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

struct Hello {
    state: StateHello,
}

enum StateHello {
    HELLO,
    WORLD,
    END,
}

impl Hello {
    fn new() -> Self {
        return Hello {
            state: StateHello::HELLO,
        };
    }
}

impl Future for Hello {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        match (*self).state {
            StateHello::HELLO => {
                println!("Hello, ");
                (*self).state = StateHello::WORLD;
                return Poll::Pending;
            }
            StateHello::WORLD => {
                println!("World!");
                (*self).state = StateHello::END;
                return Poll::Pending;
            }
            StateHello::END => {
                return Poll::Ready(());
            }
        }
    }
}

pub struct Task {
    pub hello: Mutex<BoxFuture<'static, ()>>,
}

impl Task {
    pub fn new() -> Self {
        let hello = Hello::new();
        return Task {
            hello: Mutex::new(hello.boxed()),
        };
    }
}

impl ArcWake for Task {
    fn wake_by_ref(_arc_self: &Arc<Self>) {}
}
