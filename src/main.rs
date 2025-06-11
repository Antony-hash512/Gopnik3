use std::io;

fn main() {

    let name : String;
    let life_style : i8;


    println!("20xx год от Р.Х.");
    println!("Ректор: ах ты урод, чёртов забивала!");
    println!("Ты: а ты типа чё?");
    println!("Ректор: всё, ты отчислен! Вали нахуй с универа!");
    println!("И так теперь ты из крутого пацана превратился в опущенного");

    // запрашиваем ввод у пользователя
    println!("Выбери погоняло: ");
    name = get_user_input_string();
    println!("Итак, теперь ты {}!", name);
    
    println!("Выбери кто ты по жизни: ");
    println!("1. Пацан");
    println!("2. Отморозок");
    println!("3. Нормис");
    println!("4. Спортик");
    println!("5. Чё за ботва?");
    // запрашиваем ввод цифры
    loop {
        life_style = get_user_input_i8();
        match life_style {
            1 => {
                println!("Ты пацан!");
                break;
            }
            2 => {
                println!("Ты отморозок!");
                break;
            }
            3 => {
                println!("Ты нормис!");
                break;
            }
            4 => {
                println!("Ты спортик!");
                break;
            }
            5 => {
                println!("Пацан: слабый на старте, но в два раза сильнее набирает опыт в драках");
                println!("Отморозок: более безбашенный на старте, но тугодум и тормоз в плане прокачки");
                println!("Нормис: ни рыба ни мясо, что-то среднее между пацаном и отморозком");
                println!("Спортик: не пьёт и не курит, здоровье востанавливает в качалке, качается там в три раза быстрее");
            }
            _ => {
                println!("Ты, что слепашара!? Выбери цифру от 1 до 5!");
            }
        }
    }
    show_key_map();
    loop {
        let key = get_user_input_string();
        match key {
            "g" => {
                println!("Ты шляешься по району");
            }
            "h" => {
                show_key_map();
            }
            _ => {
                println!("Ты, что слепашара!? Введи h для пояснения за шмот");
            }
        }
    }

}

fn show_key_map() {
    println!("g - шляться по району ");
    println!("h - пояснить за шмот");
    println!("i - открыть инвентарь");
    println!("e - открыть меню");
    println!("r - открыть рецепты");
    println!("t - открыть таблицу");
}

fn get_user_input_i8() -> i8 {

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Не удалось прочитать строку");
    buffer.trim().parse().expect("Please type a number!")
}

fn get_user_input_string() -> String {
    // 1. Создаем мутабельный буфер. Его "жизнь" ограничена этой функцией.
    let mut buffer = String::new();

    // 2. Используем мутабельный буфер для чтения ввода.
    // .expect() используется для простой обработки ошибок.
    io::stdin().read_line(&mut buffer).expect("Не удалось прочитать строку");

    // 3. `read_line` добавляет в конец символ новой строки (\n).
    //    Обычно его нужно убрать. Метод .trim() убирает пробельные символы
    //    с начала и конца и возвращает &str.
    //    .to_string() создает новую, владеющую строку String из этого &str.
    buffer.trim().to_string()
}