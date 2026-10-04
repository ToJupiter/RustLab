use std::pin::Pin;
use std::task::{Context, Poll};
use std::future::Future;
use std::{thread, time::Duration};


/// ``` rust
/// use std::pin::Pin;
/// use std::task::{Context, Poll};
///
/// pub trait Future {
///    type Output;
///    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
/// }
/// ```
/*
Pin<Box<F>>
    │
    │  Pin<Box<F>> IS Unpin (the box itself is just a pointer;
    │  moving the box doesn't move F, which is on the heap).
    │
    ▼
Can convert &mut Pin<Box<F>> → Pin<&mut Pin<Box<F>>>
    │
    ▼
Box<F>::poll delegates to F::poll with proper pinning.
    │
    ▼
join_all is happy.

Future<Output = ()> is a future that, when it finishes, produces nothing useful — it just finishes.
 */

type BoxFut<'a> = Pin<Box<dyn Future<Output = ()> + 'a>>;

struct JoinThree<'a> {
    futures: [BoxFut<'a>; 3],
    done: [bool; 3]
}

impl<'a> Future for JoinThree<'a> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let this = self.as_mut().get_mut();
        let mut all_done = true;
        for i in 0..3 {
            if !this.done[i] {
                if this.futures[i].as_mut().poll(cx).is_ready() {
                    this.done[i] = true;
                }
            } else {
                all_done = false;
            }
        }

        if all_done { Poll::Ready(()) } else { Poll::Pending }
    }
}

fn join_three<'a, A ,B ,C>(a: A, b: B, c: C) -> impl Future<Output = ()> + 'a
where
    A: Future<Output = ()> + 'a,
    B: Future<Output = ()> + 'a,
    C: Future<Output = ()> + 'a
{
    return JoinThree {
        futures: [Box::pin(a), Box::pin(b), Box::pin(c)],
        done: [false; 3]
    };
}

pub fn working_with_future_trait() {
    trpl::block_on(async {
        join_three(
            async { println!("a"); },
            async { println!("b"); },
            async { println!("c"); }
        ).await;
    });
}


/*
    Putting It All Together: Futures, Tasks, and Threads

    1. If the work is very parallelizable (that is, CPU-bound), such as processing a bunch of data where each part can be processed separately, threads!
    2. If the work is very concurrent (that is, I/O-bound), such as handling messages from a bunch of different sources that may come in at different intervals or different rates, async!
 */
pub fn threads_and_async() {
    let (tx, mut cx) = trpl::channel();

    thread::spawn(move || {
        for i in 1..11 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    trpl::block_on(async {
        while let Some(message) = cx.recv().await {
            println!("{}!", message);
        }
    });
}
