use rand::Rng;
use itertools::Itertools;
use eframe::egui;
use egui::Vec2;

extern crate msgbox;
use msgbox::IconType;

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.resizable = false;
    options.initial_window_size = Some(Vec2::new(256.0*1.5, 80.0*1.5));


    eframe::run_native(
        "egui",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp{
    key: String
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            key: "".to_owned()
        }
    }
}

impl eframe::App for MyApp{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame){
        ctx.set_visuals(egui::Visuals::dark());
        ctx.set_pixels_per_point(1.5);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust Win95 Keygen");
            ui.horizontal(|ui| {
                ui.label("Key: ");
                ui.label(format!("{}", self.key));
                
            });

            ui.horizontal(|ui| {
                if ui.button("Generate Key").clicked(){
                    self.key = generate_oem_key();
                }

                if ui.button("Copy to clipboard").clicked(){
                    ui.output().copied_text = String::from(&self.key);
                    msgbox::create("egui", "Key copied to clipboard!", IconType::Info).expect("Couldn't display the msgbox");
                }
            });
        });
    }
}

fn generate_oem_key() -> String{
    let mut rng = rand::thread_rng();

    let day: u32 =          rng.gen::<u32>()%366;
    let year: u8 =          (95+rng.gen::<u8>()%7)%100;
    let unchecked: u32 =    rng.gen::<u32>()%100000;
    let mut count: u32 =    0;
    let mut mod7 : Vec<u32> = (0..7).collect();
 
    loop{
        mod7[0] = 0; mod7[1] = 0;
        for n in 2..7{mod7[n] = rng.gen::<u32>()%10;}
        for n in 2..7{count+=mod7[n];}
        if count%7 == 0 && count!=0 {
            break;
        }
        else{
            count=0;
            mod7 = (0..7).collect();
        }
    }

    return format!("{:03}{:02}-OEM-{}-{:05}", day, year, mod7.iter().format(""), unchecked);
}
