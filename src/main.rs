mod scene;

use macroquad::prelude::*;
use scene::{Scene, Entity};

#[macroquad::main("MQ TACTICAL ENGINE")]
async fn main() {
    let ett = Entity{name: String::from("Test Entity"), id: 1};

    let scn = Scene {
        entities : vec!{ett},
        name: String::from("Game")
    };
    
    scn.save();

    loop {
        clear_background(BLACK);

        draw_text("MQ TACTICAL ENGINE", 20.0, 20.0, 30.0, WHITE);

        next_frame().await
    }
}