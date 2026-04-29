use std::ops::{Add};


fn add_with_traits<T: Add<Output = T>>(i: T, j: T) -> T {
  i + j                                     // разыменование указалетелей - получение значений по адресу
}


fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
  *i + *j                                   // разыменование указалетелей - получение значений по адресу
}

fn main() {


  let a = 10;
  let b = 20;
  let res = add_with_lifetimes(&a, &b);     // передача указателей на инициализированные ранее переменные
  let res_1 = add_with_traits(a, b);        // передача указателей на инициализированные ранее переменные

  println!("&a + &b  : {}", res);
  println!("a + b    : {}", res_1);
  println!("a = {}", a);                    // я думал, что здесь будет ошибка (null pointer/неинициализированная переменная)



  let res_2 = add_with_lifetimes(&10, &20); // передача указалетелей на конкретные числовые значения
  println!("&10 + &20: {}", res_2);

  

}