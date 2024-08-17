#![allow(unused)]

enum IpAddrKind {
    V4(String),
    V6(String),
}


fn main() {
    println!("O tamanho da string é: {}", string_size("Guilherme"));

    let x = 0b1111_0000;
    println!("O valor de x é: {x}");


    let mut tup: ((i32, u8), &str, f64) = ((32, 18), "rust", 64.0);
    tup.1 = "Guilherme";
    let (x, y, _) = tup;
    println!("The value of y: {y}");
    let x1 = x.1;
    println!("The value of x1: {x1}");

    let mut a = [0; 10];
    a[3] = 1;
    println!("Array: {:?}", a);

    let home: IpAddrKind = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback: IpAddrKind = IpAddrKind::V6(String::from("::1"));


}

const PI : f64 = 3.1415;

fn string_size(s: &str) -> usize {
    let s = s.len();
    s
}

fn sum (x:i32, y:i32) -> i32 {
    x + y
}