use futures::future::{BoxFuture, FutureExt};
use futures::task::{waker_ref, ArcWake};
use std::future::Future;
use std::pin::Pin;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

pub struct Task {
    // BoxFutureとは､"Pin<alloc::boxed::Box<dyn Future<Output = T> + Send + 'a>>"の型エイリアス(非常に難しい!)
    // ここでは､BoxingされたFutureと大雑把に理解しておく.
    future: Mutex<BoxFuture<'static, ()>>,
    sender: SyncSender<Arc<Task>>,
}

// ArcWakeは､実装した型をWakerとして扱えるようにするトレイト.
// https://docs.rs/futures/0.3.25/futures/task/trait.ArcWake.html
// wakerメソッドとwaker_refメソッドの2つがあり､両者の違いは引数が値型か参照か.
// ArcWakeという名の通り､Arcでラップするオブジェクトに対し使用する.
// ArcWakeはあくまで何らかの手段でwakeするためのトレイトであり､wake処理をどう実装するかは自由. ここでは実行キューにエンキューする.
impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let self0 = arc_self.clone();
        arc_self.sender.send(self0).unwrap();
    }
}

pub struct Executor {
    // ※MPSCチャンネルを実行キューに見立てている
    sender: SyncSender<Arc<Task>>,
    receiver: Receiver<Arc<Task>>,
}

impl Executor {
    pub fn new() -> Self {
        let (sender, receiver) = sync_channel(1024);
        return Executor {
            sender: sender.clone(),
            receiver,
        };
    }

    pub fn get_spawner(&self) -> Spawner {
        return Spawner {
            sender: self.sender.clone(),
        };
    }

    pub fn run(&self) {
        while let Ok(task) = self.receiver.recv() {
            let mut future = task.future.lock().unwrap();
            // waker_refメソッドにArcWakeを実装したクラスを渡すことで､wakerが取得できる
            let waker = waker_ref(&task);
            // ContextはWakerを参照するだけのシンプルなクラス
            // Futureをpollする際は､あまり深く考えずそういうもの(Context(実質Waker)を渡すのがルール)と理解する.
            let mut context = Context::from_waker(&waker);
            let poll = future.as_mut().poll(&mut context);
            match poll {
                Poll::Pending => {}
                Poll::Ready(()) => break,
            }
        }
    }
}

pub struct Spawner {
    sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    // 冒頭の関数のI/Fが非常に気持ち悪い!
    // 以下2点を理解する
    // 1. implキーワードは型の制約を示すもの. where句で以下のように書いた場合と同じ
    // https://qiita.com/kawadumax/items/580807d3f20ddd76725f#impl-trait%E3%81%A8%E3%82%B8%E3%82%A7%E3%83%8D%E3%83%AA%E3%82%AF%E3%82%B9%E3%81%AE%E9%81%95%E3%81%84
    //    pub fn spawn<F>(&self, future: F) where F: Future<Output = ()> + 'static + Send　{
    // 2. 'staticとは､'staticライフタイムのことではなくて､'staticライフタイム境界よ呼ばれるもの. 'staticライフタイム境界を明示すると､その型は非staticな参照を一切持たないことを示すらしい.
    // https://doc.rust-jp.rs/rust-by-example-ja/scope/lifetime/static_lifetime.html
    pub fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(future),
            sender: self.sender.clone(),
        });
        self.sender.send(task).unwrap();
    }
}

pub struct Hello {
    state: StateHello,
}

enum StateHello {
    HELLO,
    WORLD,
    END,
}

impl Hello {
    pub fn new() -> Self {
        return Hello {
            state: StateHello::HELLO,
        };
    }
}

impl Future for Hello {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        match (*self).state {
            StateHello::HELLO => {
                println!("Hello, ");
                (*self).state = StateHello::WORLD;
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }
            StateHello::WORLD => {
                println!("World!");
                (*self).state = StateHello::END;
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }
            StateHello::END => {
                return Poll::Ready(());
            }
        }
    }
}
