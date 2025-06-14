use std::io;
use rand::Rng;


use crate::location::Location;
use crate::fighter::Fighter;

pub struct Player {
    pub fighter : Fighter,
    pub location : Location,
    pub current_level_exp: i64,
    pub money : i64,
    pub bottles : i64,
    pub gym_is_found : bool,
    pub girl_is_found : bool,
    pub whores_is_found : bool,
    pub bar_is_found : bool,
    pub vet_is_found : bool,
    pub life_style : String,
    pub name : String,
}
impl Player {
    pub fn add_exp (&mut self,extra_exp : i64){
        let mut exp_to_add_left : i64 = extra_exp;
        let mut exp_to_add_next_level : i64 = self.fighter.level * 100;
        while exp_to_add_left > exp_to_add_next_level {
            exp_to_add_left -= exp_to_add_next_level;
            self.level_up();
            self.current_level_exp = 0;
            exp_to_add_next_level = self.fighter.level * 100;
        }
        self.current_level_exp += exp_to_add_left;
        println!("Твой опыт: {}, до следующего уровня тебе нужно ещё {} качков опыта", self.current_level_exp, exp_to_add_next_level-self.current_level_exp);
    }
    fn level_up(&mut self){
        self.fighter.level += 1;
        println!("Ты прокачал уровень, теперь твой уровень: {}", self.fighter.level)
    }
    pub fn show_info(&self){
        println!("Тебя зовут {}", self.name);
        println!("По жизни: {}", self.life_style);
        println!("Уровень: {} ({})", self.fighter.level, self.get_level_discription());
        println!("Опыт: {}, до следующего уровня тебе нужно ещё {} качков опыта", self.current_level_exp, self.fighter.level * 100 - self.current_level_exp);
        println!("Бутылки (пиво или протеин): {}", self.bottles);
        println!("Бабки: {}", self.money);

    }
    fn get_level_discription(&self) -> &str{
        match self.fighter.level {
            1 => "Полный чмо",
            2 => "Чмо",
            3 => "Почти не Чмо",
            4 => "Ещё немного и не Чмо",
            5 => "Не Чмо, но ещё лошок",
            6 => "Почти не лошок",
            7 => "Уже не лошок, но ещё не пацан",
            8 => "Ещё далеко не пацан",
            9 => "Ещё не дотягиваешь до пацана",
            10 => "Почти пацан",
            11 => "Пацан с низким авторитетом",
            12 => "Среднестатистический пацан",
            13 => "Ещё не очень крутой пацан",
            14 => "На пути к крутому пацану",
            15 => "Почти крутой пацан",
            16 => "Немного крутой пацан",
            17..=20 => "Крутой пацан",
            21..=23 => "Очень крутой пацан",
            24..=30 => "Гроза улицы",
            31..=50 => "Авторитет",
            51.. => "Криминальный авторитет",
            _ => "Неизвестный уровень"

        }
    }
}