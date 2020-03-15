#![allow(unused)]
// Serial::usart0(dp.usart0, (tx, rx, (), ()), cfg, &mut afio, &mut rcu.apb1)

struct Bundle<USART> {
    _marker: core::marker::PhantomData<USART>,
}

struct USART0;
struct PA9; struct PA10; struct PA11; struct PA12;

// impl Into<Bundle<USART0>> for (PA10, PA9, PA12, PA11) {}

trait Pins<PINS> {
    
}

impl Pins<(PA10, PA9, PA12, PA11)> for Bundle<USART0> {}

impl Pins<(PA10, PA9, (), ())> for Bundle<USART0> {}

impl Pins<(PA10, PA9, PA12, ())> for Bundle<USART0> {}

impl Pins<(PA10, PA9, (), PA11)> for Bundle<USART0> {}

struct Serial<PINS> {
    pins: PINS,
}

impl<PINS> Serial<PINS> {
    pub fn usart0(pins: PINS) -> Self
    where Bundle<USART0>: Pins<PINS> {
        todo!()
    }
}

fn main() {}
