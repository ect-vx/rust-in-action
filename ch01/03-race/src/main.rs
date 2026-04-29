use std::thread;                          // "сведение многопоточности к локальной области видимости"

fn main() {
    let mut data = 100;

    thread::spawn(|| { data = 500; });    // thread::spawn принимает замыкание
    thread::spawn(|| { data = 1000; });   // замыкание = анонимная функция = лямбда выражение

    println!("{}", data);
}