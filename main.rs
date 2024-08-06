// Author: Eser Girik
// An elegant Rust program to print Fibonacci sequence

fn main() {
    let n = 10;
    println!("Fibonacci sequence up to {} terms:", n);

    let mut fib = Fibonacci::new();
    for _ in 0..n {
        println!("{}", fib.next().unwrap());
    }
}

struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}
