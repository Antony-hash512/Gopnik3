use std::io;
use anstream::{print, println};
use owo_colors::OwoColorize as _;
use rand::Rng;

pub struct Fighter {
    pub fighter_type: String,
    pub is_npc: bool,
    pub level: i64,
    pub health: i64,
    pub max_health: i64,
    pub strength: i64,
    pub vitality: i64,
    pub accuracy: i64,
    pub agility: i64,
    pub luck: i64,
    pub intelligence: i64,
    pub willpower: i64,
    pub charisma: i64,
    pub jaw_is_broken: bool,
    pub leg_is_broken: bool,
}
impl Fighter {
    pub fn kick (&self, enemy: &mut Fighter){
        let mut damage : i64 = self.strength - enemy.vitality / 2;
        if damage < 0 { damage=0 };
        if enemy.health > damage {
            enemy.health -= damage;
            if ! self.is_npc {
                println!("{}",format!("Ты бьёшь врага на {}, у него осталось здоровья {}", damage, enemy.health).green().bold());
            } else {
                println!("{}",format!("Враг бьёт тебя на {}, у тебя остальсь {}", damage, enemy.health).red().bold());
            }
        } else {
            enemy.health = 0;
            if ! self.is_npc {
                println!("{}",format!("Ты бьёшь врага на {}, ВРАГ СДОХ", damage).green().bold());
            } else {
                println!("{}",format!("Враг бьёт тебя на {}, ТЫ СДОХ КОНЕЦ ИГРЫ", damage).red().bold());
            }
        }

    }


}