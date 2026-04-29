fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
  *i + *j                                   // разыменование указалетелей - получение значений по адресу
}

fn main() {


  let a = 10;
  let b = 20;
  let res = add_with_lifetimes(&a, &b);     // передача указателей на инициализированные ранее переменные

  println!("{}", res);

  let res_1 = add_with_lifetimes(&10, &20); // передача указалетелей на конкретные числовые значения
  println!("{}", res_1);
}