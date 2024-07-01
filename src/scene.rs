use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    pub id: u64
}

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub entities: Vec<Entity>,
    pub name: String,
}


impl Scene {
    pub fn save(&self){
        let serialized_scene = serde_json::to_string(self).unwrap();
        std::fs::write(String::from(&self.name) + ".json", serialized_scene).unwrap();
    }
}