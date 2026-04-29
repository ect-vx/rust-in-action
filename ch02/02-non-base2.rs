fn main() {
  let three = 0b11;             // '11' в двоичной системе счисления
  let thirty = 0o36;            // '36' в 8-й системе счисления
  let three_hundred = 0x12C;    // '12C' в 16-й системе счисления
  
  // способы представления чисел при выводе:

  println!("base 10: {} {} {}", three, thirty, three_hundred);
  println!("base 2:  {:b} {:b} {:b}", three, thirty, three_hundred);
  println!("base 8:  {:o} {:o} {:o}", three, thirty, three_hundred);
  println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
}
