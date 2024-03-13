use rxrust::prelude::*;

fn main() {
    let numbers = observable::from_iter(0..10);
    let even = numbers.clone().filter(|v| v % 2 == 0);
    let odd = numbers.clone().filter(|v| v % 2 != 0);
    let squared = numbers.clone().map(|v| v * v);

    even.merge(odd)
        .merge(squared)
        .subscribe(|v| print!("{} ", v,));
    // "0 2 4 6 8 1 3 5 7 9 16 25 36 49 64 81" will be printed.
}
