use std::io;
use anstream::println;
use owo_colors::OwoColorize as _;
use rand::Rng;

mod fighter;
mod player;
mod location;

use crate::player::Player;
use crate::fighter::Fighter;
use crate::location::Location;

fn main(){
    new_game();
    
}


fn new_game() { 
    let mut player : Player = Player {
        fighter : Fighter {
            fighter_type : "игрок".to_string(),
            is_npc : false,
            level : 1,
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
            leg_is_broken : false,
        },
        location : Location::Street,
        current_level_exp : 0,
        money : 0,
        bottles : 0,
        gym_is_found : false,
        girl_is_found : false,
        whores_is_found : false,
        bar_is_found : false,
        vet_is_found : false,
        life_style : "нормис".to_string(),
        name : String::new(),
    };



    println!("ГОПНИК 3: Кровь на асфальте. Rust Edition.");

    println!("20xx год от Р.Х.");
    println!("{}: ах ты урод, чёртов забивала!", "Ректор".yellow().bold());
    println!("{}: а ты типа чё?", "Ты".green().bold());
    println!("{}: всё, ты отчислен! Вали нахуй с универа!", "Ректор".yellow().bold());
    println!("{}","И так теперь ты из крутого пацана превратился в опущенного".red().bold());

    // запрашиваем ввод у пользователя
    player.name = get_user_input_string("Выбери погоняло: ");
    println!("Итак, теперь ты {}!", player.name);
    
    println!("Выбери кто ты по жизни: ");
    println!("1. Пацан");
    println!("2. Отморозок");
    println!("3. Нормис");
    println!("4. Спортик");
    println!("5. Чё за ботва?");
    // запрашиваем ввод цифры
    loop {
        let player_life_style = get_user_input_string("Введи цифру: ");
        match player_life_style.as_str() {
            "1" => {
                println!("Ты пацан!");
                player.life_style = "пацан".to_string();
                break;
            }
            "2" => {
                println!("Ты отморозок!");
                player.life_style = "отморозок".to_string();
                break;
            }
            "3" => {
                println!("Ты нормис!");
                player.life_style = "нормис".to_string();
                break;
            }
            "4" => {
                println!("Ты спортик!");
                player.life_style = "спортик".to_string();
                break;
            }
            "5" => {
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
        let key = get_user_input_string(player.location.get_prefix());
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
                player.show_info();
            }
            "q" => {
                println!("Ты выходишь из окна");
                break;
            }
            "b" => {
                if player.bottles > 0 {
                    println!("{}",format!("Ты пьёшь пиво").green().bold());
                    player.bottles -= 1;
                    player.fighter.health += 10;
                    if player.fighter.health > player.fighter.max_health {
                        player.fighter.health = player.fighter.max_health;
                    }
                    println!("Теперь у тебя {}/{} здоровья", player.fighter.health, player.fighter.max_health);
                    println!("Ты потратил 1 бутылку пива, у тебя осталось {} бутылок", player.bottles);
                } else {
                    println!("{}",format!("У тебя нет бутылок!").red().bold());
                }
            }
            "p" => {
                if player.bottles > 0 {
                    println!("{}",format!("Ты пьёшь протеин").green().bold());
                    player.bottles -= 1;
                    player.fighter.health += 10;
                    if player.fighter.health > player.fighter.max_health {
                        player.fighter.health = player.fighter.max_health;
                    }
                    println!("Теперь у тебя {}/{} здоровья", player.fighter.health, player.fighter.max_health);
                    println!("Ты потратил 1 бутылку протеина, у тебя осталось {} бутылок", player.bottles);
                } else {
                    println!("{}",format!("У тебя нет бутылок!").red().bold());
                }
                
            }
            "cheat_hp" => {
                player.fighter.health = player.fighter.max_health;
                println!("Теперь у тебя {} здоровья", player.fighter.health);
            }
            "cheat_money" => {
                player.money += 1000;
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
    if player.life_style.as_str() != "спортик" {
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

fn get_user_input_string(prefix: &str) -> String {
    use std::io::Write;
    
    // Выводим префикс перед вводом
    print!("{}", prefix);
    io::stdout().flush().unwrap(); // Принудительно выводим буфер
    
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
        is_npc : true,
        level : get_random_i64(1, 10),
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
        leg_is_broken : false,
    };
    enemy
}

fn ask_to_fight(enemy: &Fighter) -> bool{
    println!("Ты встретил врага уровня {}", enemy.level);
    println!("Будешь нарываться на врага? (y/n)");
    let input : String = get_user_input_string("");
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
            println!("{}",format!("Ты выиграл бой!").green().bold());
            println!("{}",format!("Пиво победителю!").green().bold());
            player.bottles += 2;
            // Можно добавить опыт игроку
            player.add_exp(enemy.level * 20);
            break;
        }
        
        let key = get_user_input_string("Битва> ");
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