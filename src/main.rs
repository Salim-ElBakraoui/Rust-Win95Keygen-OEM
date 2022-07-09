use rand::Rng;
use itertools::Itertools;
use eframe::egui;
use egui::Vec2;

fn main() {
    let mut options = eframe::NativeOptions::default();

    options.initial_window_size = Some(Vec2::new(256.0, 64.0));
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
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Key: ");
                ui.label(format!("{}", self.key));
            });
            if ui.button("Generate Key").clicked(){
                self.key = generate_oem_key();
            }
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
        if !(count != 0 && count%7 != 0) {
            break;
        }
    }
    let key = format!("{:03}{:02}-OEM-{}-{:05}", day, year, mod7.iter().format(""), unchecked);
    return key;
}