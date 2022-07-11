use rand::Rng;
use itertools::Itertools;
use eframe::egui;
use egui::Vec2;
use native_dialog::{MessageDialog, MessageType};

fn main() -> Result<(), Box<dyn Error>> {
    let mut options = eframe::NativeOptions::default();
    options.resizable = false;
    options.initial_window_size = Some(Vec2::new(256.0*1.25, 80.0*1.25));

    eframe::run_native(
        "egui",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );

}

/*
    Generates and returns a Windows 95 OEM Key when called
    As you can see the function is very short because Microsoft's security
    was ludicrous at best back in the days.

    Here's how it's built:
    DDDYY-OEM-MOD7KEY-UNCHK

    DDD - Day of the year (0-366) (you can use 366 even if you're not on a leap year)
    YY  - Year (95 to 02 iirc)
    OEM - OEM String
    MOD7KEY - 7 numbers. The first pair has to be 00 and then you can have any number as long as you can divide the sum by 7 ((mod7sum)%7=0)
    UNCHK - Unchecked. Literally, you can put any number there and it'll just work...
*/

fn generate_oem_key() -> String{
    let mut rng = rand::thread_rng();

    let day: u32 =          rng.gen::<u32>()%366;
    let year: u8 =          (95+rng.gen::<u8>()%7)%100;
    let unchecked: u32 =    rng.gen::<u32>()%100000;
    let mut mod7sum: u32 =    0;
    let mut mod7 : Vec<u32> = (0..7).collect();
 
    loop{
        mod7[0] = 0; mod7[1] = 0;
        for n in 2..7{mod7[n] = rng.gen::<u32>()%10;}
        for n in 2..7{mod7sum+=mod7[n];}
        if mod7sum%7 == 0 && mod7sum!=0 {
            break;
        }
        else{
            mod7sum=0;
            mod7 = (0..7).collect();
        }
    }
    return format!("{:03}{:02}-OEM-{}-{:05}", day, year, mod7.iter().format(""), unchecked);
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
        ctx.set_pixels_per_point(1.25);
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
                    MessageDialog::new()
                        .set_type(MessageType::Info)
                        .set_title("Key copied into clipboard")
                        .set_text("Done!")
                        .show_confirm()
                        .unwrap();
                }
            });
        });
    }
}

