fn main() {
  let mut letters = vec![            // создание изменяемого вектора
      "a", "b", "c"
  ];

  for letter in letters {
      println!("{}", letter);
      letters.push(letter.clone());  // копирование каждой буквы и добавление ее к концу вектора
  }
  
}
// изменение вектора при работе с ним

