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
impl Fighter {
    fn kick (&self, enemy: &mut Fighter){
        let mut damage : i64 = self.strength - enemy.vitality / 2;
        if damage < 0 { damage=0 };
        if enemy.health > damage {
            enemy.health -= damage;
            if self.fighter_type.as_str() == "игрок" {
                println!("Ты бьёшь врага на {}, у него осталось здоровья {}", damage, enemy.health);
            } else {
                println!("Враг бьёт тебя на {}, у тебя остальсь {}", damage, enemy.health)
            }
        } else {
            enemy.health = 0;
            if self.fighter_type.as_str() == "игрок" {
                println!("Ты бьёшь врага на {}, ВРАГ СДОХ", damage);
            } else {
                println!("Враг бьёт тебя на {}, ТЫ СДОХ КОНЕЦ ИГРЫ", damage)
            }
        }

    }


}