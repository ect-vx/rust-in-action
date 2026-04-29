fn greet_world() {
    println!("Hello, world!");     // использование макроса (!)

    let southern_germany = "Grüß Gott!";         // использование let для присваивания
    let japan = "ハロー・ワールド";                // нативная поддержка unicode
    // let a = 123;
    let regions = [southern_germany, japan /*, &(a.to_string())*/  ];     // создание массива

    for region in regions.iter() {               // извлечение итератора 
            println!("{}", &region);             // амперсанд "заимствует" переменную region для чтения
    }
}

fn main() {
    greet_world();                               // вызов функции
}
