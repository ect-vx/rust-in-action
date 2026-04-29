fn main() {
  let needle = 42;                 // Легаси, неиспользуемая переменная
  let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

  for item in &haystack {
    let result = match item {      // match возвращает значение, на котором произошло совпадение
      42 | 132 => "hit!",          // Получено, сообщение соотнеслось
      _ => "miss",                 // 
    };

    if result == "hit!" {
      println!("{}: {}", item, result);
    }
  }
}