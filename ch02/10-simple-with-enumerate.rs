fn main() {
  let search_term = "picture";
  let quote = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

  for (i, line) in quote.lines().enumerate() {    // используется enumerate - принимает итератор на вход и возвращает
    if line.contains(search_term) {               // номер (с нуля) и объект итерации
      let line_num = i + 1;                       // номера строк начинаются с 1, а не с 0
      println!("{}: {}", line_num, line);
    }
  }
}
