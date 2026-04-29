fn main() {
  let fruit = vec!['🥝', '🍌', '🍇'];

  let buffer_overflow = fruit[4];    // присвоение неправильной переменной

  assert_eq!(buffer_overflow, '🍉')  // ассерт - проверка правильности хода выполнения программы
}
