#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
use std::cmp::{max, min};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let waru = 1_000_000_000 + 7;
    let mut ans = 0;
    let diff = max(n, m) - min(n, m);
    if diff == 0 {
        ans = 2 * factorial(n, waru) * factorial(m, waru) % waru;
    } else if diff == 1 {
        ans = factorial(n, waru) * factorial(m, waru) % waru;
    }
    println!("{}", ans);
}
fn factorial(a: usize, waru: usize) -> usize {
    if a < 2 {
        1
    } else {
        a * (factorial(a - 1, waru) % waru) % waru
    }
}
