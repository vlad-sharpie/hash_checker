use eframe::egui;
use rfd::FileDialog;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, Read};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // logging when ran with `RUST_LOG=debug`
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 700.0]),
        ..Default::default()
    };
    eframe::run_native(
        "SHA256 Hash Checker",
        options,
        Box::new(|_cc| {
            Box::<MyApp>::default()
        }),
    )
}

struct MyApp {
    name: String,
    hash: String,
    file_path: Option<String>,
    file_hash: String,
    compare_hash: String,
    compare_result: Option<(String, egui::Color32)>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Text".to_owned(),
            hash: String::new(),
            file_path: None,
            file_hash: String::new(),
            compare_hash: String::new(),
            compare_result: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            let name_label = ui.label("Text to hash: ");
            ui.text_edit_singleline(&mut self.name)
                .labelled_by(name_label.id);

            if ui.button("Generate hash").clicked() {
                self.hash = generate_hash(&self.name);
            }

            if !self.hash.is_empty() {
                ui.label("SHA256 Hash:");
                ui.label(self.hash.clone());
            }

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Pick a file").clicked() {
                    if let Some(file_path) = FileDialog::new().pick_file() {
                        self.file_path = Some(file_path.to_string_lossy().to_string());
                        self.file_hash.clear();
                    }
                }
                if let Some(file_path) = &self.file_path {
                    ui.label(format!("File: {}", file_path));
                }
            });

            if let Some(file_path) = &self.file_path {
                if ui.button("Generate File Hash").clicked() {
                    match generate_file_hash(&file_path) {
                        Ok(hash) => self.file_hash = hash,
                        Err(err) => {
                            eprintln!("Error: {}", err);
                            self.file_hash = String::new();
                        }
                    }
                }

                if !self.file_hash.is_empty() {
                    ui.label("File SHA256 Hash:");
                    ui.label(self.file_hash.clone());
                }
            }

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Comparison Hash:");
                ui.text_edit_singleline(&mut self.compare_hash);
            });

            if let Some(file_path) = &self.file_path {
                if ui.button("Compare").clicked() {
                    let compare_result = match compare_hashes(&self.compare_hash, &file_path) {
                        Ok(result) => result,
                        Err(err) => {
                            eprintln!("Error: {}", err);
                            ("Error comparing hashes".to_owned(), egui::Color32::RED)
                        }
                    };
                    self.compare_result = Some(compare_result);
                }
            }

            if let Some((compare_result, color)) = &self.compare_result {
                ui.label("Comparison Result:");
                ui.colored_label(*color, compare_result.clone());
            }
        });
    }
}

fn generate_hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();

    format!("{:x}", result)
}

fn generate_file_hash(file_path: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut hasher = Sha256::new();
    hasher.update(&buffer);
    let result = hasher.finalize();

    Ok(format!("{:x}", result))
}

fn compare_hashes(compare_hash: &str, file_path: &str) -> Result<(String, egui::Color32), io::Error> {
    let file_hash = generate_file_hash(file_path)?;
    if compare_hash == file_hash {
        Ok(("Hashes match!".to_owned(), egui::Color32::GREEN))
    } else {
        Ok(("Hashes do not match!".to_owned(), egui::Color32::RED))
    }
}