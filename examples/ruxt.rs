use std::rc::Rc;

use ::ruxt::*;

fn main() {
    let numbers = IterObservable::from_iter(0..10);
    let even = numbers.clone().filter(|v| v % 2 == 0);
    let odd = numbers.clone().filter(|v| v % 2 != 0);
    even.subscribe(Rc::new(|val| print!("{} ", val)));
    odd.subscribe(Rc::new(|val| print!("{} ", val)));

    numbers.run();
}
