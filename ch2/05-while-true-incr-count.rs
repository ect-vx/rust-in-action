use std::time::{Duration, Instant};                // Помещение типов Duration и Instant в локальную зону видимости

fn main() {
   let mut count = 0;
   let time_limit = Duration::new(1,0);            // Создание значения продолжительности Duration, представляющего 1 секунду
   let start = Instant::now();                     // Считывание времени по системным часам

   while (Instant::now() - start) < time_limit {   // Instant - Instant = Duration
       count += 1;
   }
   println!("{}", count);
} 