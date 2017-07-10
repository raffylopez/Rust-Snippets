#[derive(Debug)]
#[allow(dead_code)]
struct Pokemon<'a> {
    name: String,
    hp_min: i32,
    hp_max: i32,
    abilities: &'a mut Vec<Ability>
}

#[allow(dead_code)]
impl<'a> Pokemon<'a> {
    fn new(name: &'static str, hp_min: i32, hp_max: i32, abilities: &'a mut Vec<Ability>)->Pokemon<'a> {
        Pokemon {name: name.to_string(), hp_min: hp_min, hp_max: hp_max, abilities: abilities}
    }
    fn print_abilities(&mut self) {
        let _handle: Vec<_> = self.abilities.into_iter().map(|a| { 
            println!("{:?}", a);
        }).collect();
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Ability {
    name: String,
    element: String,
    base_dmg: i32,
}

#[allow(dead_code)]
impl Ability {
   fn new(name: &'static str, element: &'static str, base_dmg: i32)->Ability {
       Ability {name: name.to_string(), element: element.to_string(), base_dmg: base_dmg}
   } 
}

#[derive(Debug)]
struct _Battle<'a> {
    p1: &'a Pokemon<'a>,
    p2: &'a Pokemon<'a>,
} 
impl <'a>_Battle<'a> {
    // interesting...
    fn new(pone: &'a Pokemon, ptwo: &'a Pokemon)-> _Battle<'a> {
        println!("--ROUND 1: {} is now battling {}!---", pone.name, ptwo.name );
        // saying &pone or &ptwo is redundant, since they
        // are declared references in the function anyway
        _Battle{p1: pone, p2: ptwo}
    }

    fn p1attack(&mut self, skill_name: String) {
        println!("{} attacks {} using {}!", self.p1.name, self.p2.name, skill_name );
        let _handle: Vec<_> = self.p1.abilities.iter().filter(|e| {e.name.to_string() == skill_name}).collect();
        let ability_option: Option<Ability>= _handle.iter().next();

        let ab: Ability = match ability_option {
            Some(ability) => ability,
            None=>{},
        };
        println!("{:?}", ab);
    }
}

#[allow(unused_variables)]
#[allow(unused_must_use)]
fn main() {
    let mut pikachu_abilities: &mut Vec<Ability> = &mut vec![Ability::new("LightningBolt", "Fire", 20)];
    pikachu_abilities.push(Ability::new("Taze", "Fire", 10));
    let mut pikachu = Pokemon::new("Pikachu", 200, 245, pikachu_abilities);
    pikachu.abilities.push(Ability::new("Thunderstorm", "Fire", 100));
    println!("{:?}", pikachu);
    pikachu.print_abilities();

    let mut blastoise_abilities: &mut Vec<Ability> = &mut vec![Ability::new("Splash", "Water", 5)];
    let blastoise = Pokemon::new("Blastoise", 200, 245, blastoise_abilities);

    let mut battle = &mut _Battle::new(&pikachu, &blastoise);
    println!("{:?}", &mut battle.p1);

    battle.p1attack("LightningBolt".to_string());


}
