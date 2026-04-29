#[derive(Debug)]    // ?? разрешение макросу println! прочитать содержимое (или структуру) enum
enum Cereal {       // создание перечисления (перечисление - структура, имеющая фиксированное количество допустимых значений)
    Barley, Millet, Rice,
    Rye, Spelt, Wheat,
}

fn main() {
    let mut grains: Vec<Cereal> = vec![];   // инициализация пустого вектора
    grains.push(Cereal::Rye);               // добавление значения (пуш) в вектор
    drop(grains);                           // очищение вектора полностью

    println!("{:?}", grains);               // попытка чтения вектора
}
