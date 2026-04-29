fn main() {             // отключение завершающего символа строки
  let penguin_data = "\
common name,length (cm)
Little penguin,33  
  Yellow-eyed penguin,65
Fiordland penguin,   60
Invalid  ,data";

  let records = penguin_data.lines();

  // for (i, record) in records.enumerate() {
  //   println!("{}    :    {}", i, record);
  // }
  // print!("\n\n\n\n");

  for (i, record) in records.enumerate() {
    if i == 0 || record.trim().len() == 0 {  // пропуск пустых строк и первой
      continue;
    }

    let fields: Vec<_> = record     // начало со строки текста  
      .split(',')                   // разбиение записей на поля
      .map(|field| field.trim())    // удаление пробелов
      .collect();                   // "сборка набора полей"

    if cfg!(debug_assertions) {              // проверка флагов компиляции. cfg! проверяет конфигурацию в процессе компиляции
      eprintln!("debug: {:?} -> {:?}", record, fields);            // вывод в поток ошибок (2)
    }

    let name = fields[0];
    if let Ok(length) = fields[1].parse::<f32>() { // попытка парсинга поля как значения f32 - в случае успеха - запись в length 


        println!("{}, {}cm", name, length);
    }
  }
}
