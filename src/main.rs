use std::io;
use rand::Rng;

mod fighter;
mod player;

use crate::player::Player;
use crate::fighter::Fighter;

fn main(){
    new_game();
    
}


fn new_game() {
    
    let mut player : Player = Player {
        fighter : Fighter {
            fighter_type : "игрок".to_string(),
            level : 1,
            exp : 0,
            health : 150,
            max_health : 150,
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
        if player.fighter.health <= 0 {
            println!("ТЫ СДОХ, КОНЕЦ ИГРЫ");
            break;            
        }

        let key = get_user_input_string();
        match key.as_str() {
            "g" => {
                println!("Ты шляешься по району");
                get_event(&mut player);
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
    rng.random_range(min..max)
}

fn spown_enemy() -> Fighter{
    let enemy = Fighter {
        fighter_type : String::new(),
        level : get_random_i64(1, 10),
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
    };
    enemy
}

fn ask_to_fight(enemy: &Fighter) -> bool{
    println!("Ты встретил врага уровня {}", enemy.level);
    println!("Будешь нарываться на врага? (y/n)");
    let input : String = get_user_input_string();
    let answer: &str = input.as_str();
    if answer == "y" {
        println!("Эй, пацан, ты из какого района?");
        println!("А ты по пинкам суди!");
        return true;
    } else {
        println!("Ты не нарываешься на врага, и продолжаешь шляться по району");
        return false;
    }
}

fn get_event(player : &mut Player){
    let event_type = get_random_i64(1, 3);
    match event_type {
        2 => {
            let mut enemy = spown_enemy();
            if ask_to_fight(&enemy) {
                println!("Ты вступаешь в бой с врагом");
                battle(player, &mut enemy)
            } else {
                println!("Ты не вступаешь в бой с врагом");
            }
        }
        _ => {
            println!("Ничего не произошло");
        }
    }
}

fn battle(player: &mut Player, enemy: &mut Fighter){
    println!("Начался бой! У тебя {} здоровья, у врага {} здоровья", player.fighter.health, enemy.health);
    println!("k - ударить, h - чё за ботва?, r - убежать");
    
    loop{
        // Проверяем, не умер ли кто-то
        if player.fighter.health <= 0 {
            println!("Ты проиграл бой!");
            break;
        }
        if enemy.health <= 0 {
            println!("Ты выиграл бой!");
            // Можно добавить опыт игроку
            player.add_exp(enemy.level * 20);
            break;
        }
        
        let key = get_user_input_string();
        match key.as_str() {
            "k" => {
                player.fighter.kick(enemy);
                if enemy.health > 0 {
                    enemy.kick(&mut player.fighter);
                }
            }
            "h" => {
                println!("k - ударить врага");
                println!("r - убежать из боя");
                println!("h - показать эту помощь");
            }
            "r" => {
                println!("Ты такой ссыкло, что смог сбежать из боя!");
                break;
            }
            _ => {
                println!("Неизвестная команда! Нажми h для помощи");
            }
        }
    }
}