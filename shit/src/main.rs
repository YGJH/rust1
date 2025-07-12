struct Cat {
    name: String,
    age: u8
}

fn main() {
    let kitty = Cat { name: "Kitty".to_string(), age: 12 };
    let nancy = Cat { name: "Nancy".to_string(), age: 16 };

    let boss = boss_cat(&kitty, &nancy);
    println!("{}", boss.name);
}

fn boss_cat<'a>(c1: &'a Cat, c2: &'a Cat) -> &'a Cat {
    if c1.age > c2.age {
        c1
    } else {
        c2
    }
}