use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
  let f = File::open("readme.md").unwrap();     // создание объекта File, получение результата 
                                                // из структуры "Result" для безопасности
  let mut reader = BufReader::new(f);

  let mut line = String::new();    // создание изменяемого объекта для хранения строк

  loop {
    let len = reader.read_line(&mut line)
                    .unwrap();  // явная обработка "опасной" операции
                                // - чтение с диска может быть проблемой
    if len == 0 {
      break
    }

    println!("{} ({} bytes long)", line, len);

    line.truncate(0);    // 
  }
}