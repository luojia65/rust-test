use core::future::Future;
use core::task::{Context, Poll, Waker};
use core::pin::Pin;

struct LazerGun {
    _private: ()
}

impl LazerGun {
    async fn charge(self) -> Ready {
        println!("Charging!");
        Charge::Prepared(self, 5).await
    }
}

enum Charge {
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

struct Ready {
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

async fn two_guns(a: LazerGun, b: LazerGun) -> (Ready, Ready) {
    let a = a.charge().await;
    let b = b.charge().await;
    (a, b)
}

#[async_std::main]
async fn main() {
    let g1 = LazerGun { _private: () };
    let g2 = LazerGun { _private: () };
    let (mut a, mut b) = two_guns(g1, g2).await;
    for _ in 0..5 {
        a.shoot();
        b.shoot();
    }
    a.free();
    b.free();
}
