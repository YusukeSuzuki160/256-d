// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        r:[(usize, usize); n],
    }
    let mut query = Vec::new();
    for (l, r) in r {
        query.push((l, 0));
        query.push((r, 1));
    }
    query.sort();
    let mut cnt = 0;
    for (t, f) in query {
        if f == 0 {
            if (cnt == 0) {
                print!("{} ", t);
            }
            cnt += 1;
        } else {
            cnt -= 1;
            if cnt == 0 {
                println!("{}", t);
            }
        }
    }
}
