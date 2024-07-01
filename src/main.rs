mod scene;

use egui::Align2;
use macroquad::prelude::*;
use scene::{Scene, Entity};

#[macroquad::main("MQ TACTICAL ENGINE")]
async fn main() {
    // Persistent editor UI

    let mut show_about_window = false;


    // Temp scene saving
    let ett = Entity{name: String::from("Test Entity"), id: 1};

    Scene::load(String::from("Game.json"));

    let mut scn = Scene {
        entities : vec!{ett},
        name: String::from("Game"),
        path: String::new()
    };
    
    scn.save();


    // editor
    
    loop {
        clear_background(BLACK);
        
        draw_text("MQ TACTICAL ENGINE", 20.0, 20.0, 30.0, WHITE);
        egui_macroquad::ui(|egui_ctx| {

            
            
            egui::TopBottomPanel::top("action_bar").show(egui_ctx,|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.button("File");
                    ui.button("Edit");
                    
                    
                    let about = ui.button("About");
                    
                    if about.clicked(){
                        show_about_window = true;
                    }

                    if show_about_window == true{
                        egui::Window::new("About")
                            .anchor(Align2::CENTER_CENTER, [0.0,0.0])
                            .collapsible(false)
                            .resizable(false)
                            .open(&mut show_about_window)
                            .show(egui_ctx, |ui|{
                            ui.label("MQ Tactical Engine - indev");
                        });
                    }
                })
             });

            egui::SidePanel::new(egui::panel::Side::Left, egui::Id::new("scene_tree")).show(egui_ctx, |ui|{
                ui.label("Scene Tree");
            });
        });


        egui_macroquad::draw();

        next_frame().await
    }
}