use std::{io::{self,Write}, time::Duration};
use crossterm::{
    event::{KeyCode, Event }, terminal::{enable_raw_mode, disable_raw_mode},
};

enum RobotCmd {
    MoveLocal { x: f32, y: f32, omega: f32 },
    Stop,
}

struct RobotClient {
    send_cmd: std::sync::mpsc::Sender<RobotCmd>,
}

impl RobotClient {
    fn new(port_name: &str) -> RobotClient {
        let (tx, rx) = std::sync::mpsc::channel();
        let port_name = port_name.to_owned();
        std::thread::spawn(move || {
            let mut port = serialport::new(port_name, 115200)
                .timeout(Duration::from_millis(10))
                .open()
                .expect("Failed to open port");

            let mut current_cmd: Option<RobotCmd> = None;
            loop {
                match rx.try_recv() {
                    Ok(cmd) => {
                        current_cmd = Some(cmd);
                    }
                    Err(_) => {
                        // No command
                    }
                }

                match current_cmd {
                    Some(RobotCmd::MoveLocal { x, y, omega }) => {
                        let cmd = format!("Sy{x};Sx{y};Sz{omega};S.\n");
                        port.write_all(cmd.as_bytes())
                            .expect("Failed to write to port");
                    }
                    Some(RobotCmd::Stop) => {
                        // Noop
                    }
                    None => {}
                }

                std::thread::sleep(Duration::from_secs(1) / 20);
            }
        });

        RobotClient { send_cmd: tx }
    }

    fn send(&self, cmd: RobotCmd) {
        self.send_cmd.send(cmd).expect("Failed to send command");
    }
}

fn list_ports() -> Vec<String> {
    let ports = serialport::available_ports().expect("No ports found!");
    ports.into_iter().map(|p| p.port_name).collect()
}

fn main() {
    enable_raw_mode().expect("Failed to enable raw mode");
    loop {
        match crossterm::event::read() {
            Ok(Event::Key(event)) => {
                // Check for arrow keys
                match event.code {
                    KeyCode::Char('w') => {
                        println!("Up");
                    }
                    KeyCode::Char('s') => {
                        println!("Down");
                    }
                    KeyCode::Char('a') => {
                        println!("Left");
                    }
                    KeyCode::Char('d') => {
                        println!("Right");
                    }
                    KeyCode::Char('q') => {
                        println!("Quit");
                        break;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    disable_raw_mode().expect("Failed to disable raw mode");
}
