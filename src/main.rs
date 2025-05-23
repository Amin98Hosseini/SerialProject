#![windows_subsystem = "windows"]  // Add this line at the very top of main.rs

use eframe::egui;
use serialport::SerialPort;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Serial Port GUI",
        options,
        Box::new(|_cc| Ok(Box::new(SerialGui::default())))
    )
}

struct SerialGui {
    available_ports: Vec<String>,
    selected_port: Option<String>,
    baud_rate: u32,
    send_buffer: String,
    receive_buffer: Arc<Mutex<String>>,
    serial_thread_handle: Option<thread::JoinHandle<()>>,
    serial_writer: Option<Arc<Mutex<Box<dyn SerialPort>>>>,
    is_connected: bool,
    status_message: String,
    running: Arc<Mutex<bool>>,  // Add this new field
}

impl Default for SerialGui {
    fn default() -> Self {
        let ports = serialport::available_ports()
            .unwrap_or_default()
            .into_iter()
            .map(|p| p.port_name)
            .collect();
        Self {
            available_ports: ports,
            selected_port: None,
            baud_rate: 9600,
            send_buffer: String::new(),
            receive_buffer: Arc::new(Mutex::new(String::new())),
            serial_thread_handle: None,
            serial_writer: None,
            is_connected: false,
            status_message: String::new(),
            running: Arc::new(Mutex::new(true)),  // Initialize the new field
        }
    }
}

impl SerialGui {
    fn disconnect(&mut self) {
        // Signal the thread to stop
        if let Ok(mut running) = self.running.lock() {
            *running = false;
        }
        
        // Drop the writer to close the port
        if let Some(writer) = self.serial_writer.take() {
            drop(writer);
        }
        
        // Wait for thread to finish
        thread::sleep(Duration::from_millis(100));
        if let Some(handle) = self.serial_thread_handle.take() {
            let _ = handle.join();
        }
        
        self.is_connected = false;
        self.status_message = "Disconnected".to_string();
    }
}

impl eframe::App for SerialGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Serial Port Communication");

            ui.horizontal(|ui| {
                // Port selection
                egui::ComboBox::from_label("Select Port")
                    .selected_text(self.selected_port.clone().unwrap_or_else(|| "None".to_string()))
                    .show_ui(ui, |ui| {
                        for port in &self.available_ports {
                            ui.selectable_value(&mut self.selected_port, Some(port.clone()), port);
                        }
                    });

                // Refresh ports button
                if ui.button("🔄").clicked() {
                    self.available_ports = serialport::available_ports()
                        .unwrap_or_default()
                        .into_iter()
                        .map(|p| p.port_name)
                        .collect();
                }
            });

            ui.horizontal(|ui| {
                ui.label("Baud Rate:");
                ui.add(egui::DragValue::new(&mut self.baud_rate).range(300..=115200));
            });

            ui.horizontal(|ui| {
                if !self.is_connected {
                    if ui.button("Connect").clicked() && self.selected_port.is_some() {
                        if let Some(ref port_name) = self.selected_port {
                            match serialport::new(port_name, self.baud_rate)
                                .timeout(Duration::from_millis(100))
                                .open()
                            {
                                Ok(port) => {
                                    let port = Arc::new(Mutex::new(port));
                                    let recv_buffer = self.receive_buffer.clone();
                                    let reader_port = Arc::clone(&port);
                                    let running = Arc::clone(&self.running);
                                    
                                    // Set running to true before starting thread
                                    if let Ok(mut is_running) = running.lock() {
                                        *is_running = true;
                                    }

                                    self.serial_thread_handle = Some(thread::spawn(move || {
                                        let mut buf = [0u8; 128];
                                        while let Ok(is_running) = running.lock() {
                                            if !*is_running {
                                                break;
                                            }
                                            drop(is_running);

                                            if let Ok(mut port) = reader_port.lock() {
                                                match port.read(&mut buf) {
                                                    Ok(n) => {
                                                        if let Ok(text) = String::from_utf8(buf[..n].to_vec()) {
                                                            if let Ok(mut rx) = recv_buffer.lock() {
                                                                rx.push_str(&text);
                                                            }
                                                        }
                                                    }
                                                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {}
                                                    Err(_) => {
                                                        if let Ok(mut is_running) = running.lock() {
                                                            *is_running = false;
                                                        }
                                                        break;
                                                    }
                                                }
                                            }
                                            thread::sleep(Duration::from_millis(10));
                                        }
                                    }));
                                    self.serial_writer = Some(port);
                                    self.is_connected = true;
                                    self.status_message = "Connected".to_string();
                                }
                                Err(e) => {
                                    self.status_message = format!("Connection error: {}", e);
                                }
                            }
                        }
                    }
                } else {
                    if ui.button("Disconnect").clicked() {
                        self.disconnect();
                    }
                }

                ui.label(&self.status_message);
            });

            // Send area
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    let text_edit = ui.text_edit_singleline(&mut self.send_buffer);
                    if (ui.button("Send").clicked() || text_edit.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                        && !self.send_buffer.is_empty()
                    {
                        if let Some(ref writer) = self.serial_writer {
                            let mut data = self.send_buffer.clone();
                            data.push('\n');
                            let mut port = writer.lock().unwrap();
                            if port.write_all(data.as_bytes()).is_ok() {
                                self.send_buffer.clear();
                            }
                        }
                    }
                });
            });

            // Received area
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Received Data:");
                    if ui.button("Clear").clicked() {
                        if let Ok(mut buffer) = self.receive_buffer.lock() {
                            buffer.clear();
                        }
                    }
                });

                let mut received = self.receive_buffer.lock().unwrap().clone();
                let response = ui.add(
                    egui::TextEdit::multiline(&mut received)
                        .desired_rows(10)
                        .desired_width(f32::INFINITY)
                );
                
                if response.changed() {
                    if let Ok(mut buffer) = self.receive_buffer.lock() {
                        *buffer = received;
                    }
                }
            });
        });

        // Request repaint frequently to update received data
        ctx.request_repaint_after(Duration::from_millis(100));
    }
}
