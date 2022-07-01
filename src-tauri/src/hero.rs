use serde;
use serde::{ser, Serialize, Serializer};
use serde::ser::SerializeStruct;

pub struct Hero{
    name: String,
    hp: String,
}

impl Serialize for Hero{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer,
    {
        let mut s = serializer.serialize_struct("Hero", 2)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("hp", &self.hp)?;
        s.end()
    }
}

impl Hero{
    pub fn get_hero(&self) -> String{
        let text = format!("My {} with HP:{}", self.name, self.hp);
        text
    }
    
    pub fn new(name: String, hp: String) -> Hero{
        Hero{name, hp }
    }
}

#[tauri::command]
pub fn create_hero(name: &str, hp: &str) -> Hero {
    let new_hero = Hero::new(name.to_string(),hp.to_string());
    let text = new_hero.get_hero();
    println!("{}", text);
    new_hero
    
}