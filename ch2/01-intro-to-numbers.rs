fn main() {
  let twenty = 20;                       // автоматическое определение типа
  let twenty_one: i32 = 21;              // аннотация для определения типа
  let twenty_two = 22i32;                // суффикс для определения типа

  let addition = twenty + twenty_one + twenty_two;
  println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

  let one_million: i64 = 1_000_000;      // "_" игнорируются компилятором в числовых значениях
  println!("{}", one_million.pow(2));    // у чисел есть методы

  let forty_twos = [                     //<6>
    42.0,                                //<7>
    42f32,                               //<8>
    42.0_f32,                            //<9>
  ];
  
  println!("{}", forty_twos[0]);
  println!("{:02}", forty_twos[0]);      //<10>
}
