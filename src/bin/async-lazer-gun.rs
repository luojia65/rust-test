use core::future::Future;
use core::task::{Context, Poll, Waker};
use core::pin::Pin;

pub struct LazerGun {
    _private: ()
}

impl LazerGun {
    pub async fn charge(self) -> Ready {
        println!("Charging!");
        Charge::Prepared(self, 5).await
    }
}

pub enum Charge {
    Prepared(LazerGun, usize),
    Processing(LazerGun, usize, Waker),
    Ready(LazerGun, Waker),
    Done
}

impl Future for Charge {
    type Output = Ready;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let this = self.get_mut();
        let state = core::mem::replace(this, Charge::Done);
        match state {
            Charge::Prepared(g, cnt) => {
                *this = Charge::Processing(g, cnt - 1, cx.waker().clone());
                cx.waker().clone().wake();
                Poll::Pending
            },
            Charge::Processing(g, cnt, waker) => {
                if cnt == 0 {
                    println!("Ready!");
                    *this = Charge::Ready(g, cx.waker().clone());
                } else {
                    println!("Still charging! Remaining: {}", cnt);
                    *this = Charge::Processing(g, cnt - 1, cx.waker().clone());
                }
                waker.wake();
                Poll::Pending
            },
            Charge::Ready(g, waker) => {
                waker.wake();
                Poll::Ready(Ready { g, remaining: 3 })
            },
            Charge::Done => panic!("poll a future that's done") 
        }
    }
}

pub struct Ready {
    g: LazerGun,
    remaining: usize
}

impl Ready {
    pub fn shoot(&mut self) {
        if self.remaining == 0 {
            println!("Oops! No remaining shoots available")
        } else {
            self.remaining -= 1;
            println!("Biu!")
        }
    }
    
    pub fn free(self) -> LazerGun {
        println!("Unarmed!");
        self.g
    }
}

#[async_std::main]
async fn main() {
    let g = LazerGun { _private: () };
    let mut ready = g.charge().await;
    for _ in 0..5 {
        ready.shoot();
    }
    let _ = ready.free();
}
