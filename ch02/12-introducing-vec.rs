fn main() {
  let ctx_lines = 2;
  let needle = "oo";
  let haystack = "\
Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";

  let mut tags: Vec<usize> = vec![];               // хранятся номера совпадающих строк
  let mut ctx: Vec<Vec<(
               usize, String)>> = vec![];          // в каждом вложенном векторе хранятся строки до и после

  for (i, line) in haystack.lines().enumerate() {  // использование enumerate для удобной работы с итератором
    if line.contains(needle) {
      tags.push(i);                                // добавление номера строки с совпадением

      let v = Vec::with_capacity(2*ctx_lines + 1); // резервирование места под  строки до и после совпадения
      ctx.push(v);
    }
  }

  if tags.is_empty() {                             // выход если соответствий нет
    return;
  }

  for (i, line) in haystack.lines().enumerate() {  // проверка "близости" строки для её вывода на экран
    for (j, tag) in tags.iter().enumerate() {
      let lower_bound =
	      tag.saturating_sub(ctx_lines);           // 1.saturating_sub(5) = 0, безопасное вычитание
      let upper_bound =
	      tag + ctx_lines;

      if (i >= lower_bound) && (i <= upper_bound) {
          let line_as_string = String::from(line); // копирование строк в новое значение "String"
          let local_ctx = (i, line_as_string);
          ctx[j].push(local_ctx);
      }
    }
  }

  for local_ctx in ctx.iter() {
    for &(i, ref line) in local_ctx.iter() {       // ref - говорит, что нужно не перемещать, а заимствовать
      let line_num = i + 1;
      println!("{}: {}", line_num, line);
    }
    println!();
  }
}
