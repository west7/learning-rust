#![allow(unused)]
struct Account {
    balance: i32,
    limit: i32,
}

#[derive(Debug)]
struct Unit();

fn main() {
    let mut account = Account { limit: 5000, balance: 10000};
}

