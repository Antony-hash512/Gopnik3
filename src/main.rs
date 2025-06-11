use std::io;
use rand::Rng;

struct Fighter {
    fighter_type : String,
    level : i64,
    exp : i64,
    health : i64,
    max_health : i64,
    strength : i64,
    vitality : i64,
    accuracy : i64,
    agility : i64,
    luck : i64,
    intelligence : i64,
    willpower : i64,
    charisma : i64,
    jaw_is_broken : bool,

}
struct Player {
    fighter : Fighter,
    money : i64,
    bottles : i64,
    gym_is_found : bool,
    girl_is_found : bool,
    whores_is_found : bool,
    bar_is_found : bool,
    vet_is_found : bool,
    life_style : i8,
    name : String,
}

fn main() {
    



    let mut player : Player = Player {
        fighter : Fighter {
            fighter_type : String::new(),
            level : 1,
            exp : 0,
            health : 100,
            max_health : 100,
            strength : 10,
            vitality : 10,
            accuracy : 10,
            agility : 10,
            luck : 10,
            intelligence : 1,
            willpower : 100,
            charisma : 10,
            jaw_is_broken : false,
        },
        money : 0,
        bottles : 0,
        gym_is_found : false,
        girl_is_found : false,
        whores_is_found : false,
        bar_is_found : false,
        vet_is_found : false,
        life_style : 3,
        name : String::new(),
    };



    println!("ГОПНИК 3: Кровь на асфальте. Rust Edition.");

    println!("20xx год от Р.Х.");
    println!("Ректор: ах ты урод, чёртов забивала!");
    println!("Ты: а ты типа чё?");
    println!("Ректор: всё, ты отчислен! Вали нахуй с универа!");
    println!("И так теперь ты из крутого пацана превратился в опущенного");

    // запрашиваем ввод у пользователя
    println!("Выбери погоняло: ");
    player.name = get_user_input_string();
    println!("Итак, теперь ты {}!", player.name);
    
    println!("Выбери кто ты по жизни: ");
    println!("1. Пацан");
    println!("2. Отморозок");
    println!("3. Нормис");
    println!("4. Спортик");
    println!("5. Чё за ботва?");
    // запрашиваем ввод цифры
    loop {
        player.life_style = get_user_input_i8();
        match player.life_style {
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
                println!("1. Пацан: слабый на старте, но в два раза сильнее набирает опыт в драках");
                println!("2. Отморозок: более безбашенный на старте, но тугодум и тормоз в плане прокачки");
                println!("3. Нормис: ни рыба ни мясо, что-то среднее между пацаном и отморозком");
                println!("4. Спортик: не пьёт и не курит, здоровье востанавливает протеином, сразу знает путь в качалку");
            }
            _ => {
                println!("Ты, что слепашара!? Выбери цифру от 1 до 5!");
            }
        }
    }
    show_key_map(&player);
    loop {
        let key = get_user_input_string();
        match key.as_str() {
            "g" => {
                println!("Ты шляешься по району");
            }
            "h" => {
                show_key_map(&player);
            }
            "k" => {
                println!("Ты чё тут просто так копытами размахиваешь? Сначала найди кого пинать!");
            }
            "s" => {
                println!("Ты смотришь в лужу на свою рожу");
            }
            "q" => {
                println!("Ты выходишь из окна");
                break;
            }
            _ => {
                println!("Ты, что слепашара!? Введи h для того чтобы разуть глаза");
            }
        }
    }

}

fn show_key_map(player : &Player) {
    println!("g - шляться по району ");
    println!("h - разуть глаза (вспомнтить что ты можешь)");
    println!("s - посмотреть в лужу на свою рожу");
    println!("k - пинать какого-то отморозка");
    if player.life_style != 4 {
        println!("b - пить пиво");
    } else {
        println!("p - пить протеин");
    }
    if player.gym_is_found {
        println!("g - пойти в качалку");
    }
    if player.bar_is_found {
        println!("bar - пойти в бар");
    }
    if player.whores_is_found {
        println!("whores - пойти к шлюхам");
    }
    if player.girl_is_found {
        println!("girl - пойти к своей чике");
    }
    if player.vet_is_found {
        println!("vet - пойти к ветеринару");
    }
    println!("q - выйти из игры");
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

fn get_random_i64(min : i64, max : i64) -> i64 {
    let mut rng = rand::rngs::ThreadRng::default();
    rng.gen_range(min..max)
}

fn spown_enemy(){
    let enemy_level = get_random_i64(1, 10);

    println!("Будешь нарываться на врага? (y/n)");
    let input : String = get_user_input_string();
    let answer: &str = input.as_str();
    if answer == "y" {
        println!("Эй, пацан, ты из какого района?");
        println!("А ты по пинкам суди!");
    } else {
        println!("Ты не нарываешься на врага, и продолжаешь шляться по району");
    }
}

fn get_event(){
    let event_type = get_random_i64(1, 10);
    match event_type {
        1 => {
            spown_enemy();
        }
        _ => {
            println!("Ничего не произошло");
        }
    }
}