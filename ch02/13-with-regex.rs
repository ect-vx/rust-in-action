---
[package]
name = "grep-lite"
version = "0.1.0"
edition = "2024"

[dependencies]
regex = "1.10"
clap = "2.34"
---

/* 
Выше представлена магия "карго скрипт", появившаяся в 1.80 
в начале 2026 года. Позволяет проще управлять зависимостями
при работе вне проекта - с отдельными скриптами. Ранееs
данный функционал был доступен с помощью сторонних инструментов
и в тестовой ветке.
*/

use regex::Regex;    // перемещение типов Regex, App, Arg
use clap::{App,Arg}; // в локальную зону видимости

fn main() {
  let args = App::new("grep-lite")      // создание синтаксического анализатора 
    .version("0.1")                     // аргументов команды путём модификации объекта 
    .about("regex match")               // App 
    .arg(Arg::with_name("pattern")
      .help("regex pattern to search for")
      .takes_value(true)
      .required(true))
    .get_matches();

  let pattern = args.value_of("pattern").unwrap(); // получение аргумента из объекта типа App
  let re = Regex::new(pattern).unwrap();    // "unwrap" разворачивает возвращаемый
                                              // тип Result и помогает при ошибках
  let quote = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

  for line in quote.lines() {
    let contains_substring = re.find(line);
    match contains_substring {    // замена "contains()" структурой match для обработки различных случаев

        Some(_) => println!("{}", line),    // Some(T) - положительный вариант значения Option,
                                            // означающий, что работа метода завершена успешно
        None => (),    // Отричательый вариант значения типа Option - ничего не нашлось.
                       // unit тип () может рассматриваться как null значение заместителя
    }
  }
}
