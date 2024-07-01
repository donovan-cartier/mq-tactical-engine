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
    pub path: String,
}


impl Scene {
    pub fn save(&mut self){
        let serialized_scene = serde_json::to_string(self).unwrap();
        std::fs::write(String::from(&self.name) + ".json", serialized_scene).unwrap();
        self.path = String::from(&self.name);
    }

    pub fn load(path: String){
        let scene = std::fs::read(path).unwrap();
        // Implement file check here

        println!("{}", String::from_utf8(scene).unwrap());
        // println!("loading {scene_name} from {scene_path}", scene_name = self.name, scene_path = self.path);
    }
}