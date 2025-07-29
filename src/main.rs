mod utils;

use std::env::{current_dir, set_current_dir};
use std::fmt::format;
use std::path;
use std::path::{PathBuf, Path};

use iced::{widget::{button, column, text, row}, window, Color, Element, Fill, Subscription, Task, Theme, keyboard};
use iced::keyboard::key::Named;
use iced::widget::{horizontal_rule, Column};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Message {
    Exit,
    Up,
    Down,
    Enter,
    Ret,
}
const MAX_FILES: usize = 22;
const RED: Color = Color::from_rgb(1.0, 0.0, 0.0);
const BLUE: Color = Color::from_rgb(0.0, 0.0, 1.0);
const GREEN: Color = Color::from_rgb(0.0, 1.0, 0.0);


#[derive(Debug)]
struct AppState {
    current_dir: PathBuf,
    current_files: Vec<(String,bool)>,
    selected_file: i8,
    success: bool,
    ran: bool,
}

impl Default for AppState {
    fn default() -> Self {
        let current_dir: PathBuf = current_dir().unwrap();
        let current_files: Vec<(String,bool)> = utils::get_files(&current_dir);
        
        AppState{
            current_dir,
            current_files,
            selected_file: 0,
            success: false,
            ran: false
        }
    }
}

impl AppState {
    

    fn view(&self) -> Element<Message> {
        let file = format!("{} ", self.current_files[self.selected_file as usize].0);
        if self.ran{
            let (col,val) = if self.success {
                (GREEN, "Successful Audio Rip")
            }else{
                (RED, "Audio Rip Failed")
            };
            return row![
                text("File 2 Rip: ").size(25),
                text(file).size(25).color(BLUE),
                text(val).size(25).color(col).width(Fill),
                button("Click Me To Return To JRIP").on_press(Message::Ret),
               
                
            ].into();
            
        };
        
        // Top row: current directory + Exit button
        let header = row![
            text(self.current_dir.to_str().unwrap()).size(24).width(Fill),
            button(text("Exit").size(20)).on_press(Message::Exit),
            
        ].spacing(8);
        
        let mut columns: Vec<Column<Message>> = vec![];
        let mut current_column = column![];
        let mut count = 0;

        for val in &self.current_files {
            let color = if val.0 == self.current_files[self.selected_file as usize].0 {
                RED
            } else if val.1 {
                BLUE
            } else {
                GREEN
            };
            
            current_column = current_column.push(
                text(val.0.clone()).size(24).color(color)
            );
            count += 1;

            
            if count == MAX_FILES {
                columns.push(current_column);
                current_column = column![];
                count = 0;
            }
        }
        
        if count > 0 {
            columns.push(current_column);
        }

        let file_columns = row(
            columns.into_iter().map(|col| col.into()).collect::<Vec<_>>()
        ).spacing(16);
        
        column![
            header.width(Fill),
            horizontal_rule(2),
            file_columns
        ]
        .spacing(16)
        .into()
    }


    fn update (&mut self, msg: Message) -> Task<Message>{
            match msg {
                Message::Exit => {
                    window::get_latest().and_then(window::close)
                },
                Message::Up => {
                    if !self.current_files.is_empty() {
                        if self.selected_file > 0 {
                            self.selected_file -= 1;
                        } else {
                            self.selected_file = (self.current_files.len() -1) as i8;
                        }
                    }
                    Task::none()
                },
                Message::Down => {
                    if !self.current_files.is_empty() {
                        if self.selected_file < (self.current_files.len() -1) as i8 {
                            self.selected_file += 1;
                        } else {
                            self.selected_file = 0
                        }
                        
                        
                    }
                    Task::none()
                },
                Message::Enter => {
                    let path = path::absolute(
                        PathBuf::from(
                            self.current_files[self.selected_file as usize].
                                0
                                .clone()
                        )
                    ).unwrap();
                    
                    if path.is_dir() {
                        set_current_dir(&path).unwrap();
                        self.current_dir = path::absolute(&path).unwrap();
                        self.selected_file = 0;
                        self.current_files = utils::get_files(&self.current_dir);
                    }else{
                        self.ran = true;
                        self.success = utils::rip_it(
                            &self.current_files[self.selected_file as usize].0,
                            &self.current_dir
                        );
                    }
                    Task::none()
                },
                Message::Ret => {
                    self.ran = false;
                    self.success = false;
                    Task::none()
                }
            }
    }

    fn subscription(&self) -> Subscription<Message> {
        fn handle_hotkey(key: keyboard::Key, _modifiers: keyboard::Modifiers) -> Option<Message> {
            match key.as_ref() {
                keyboard::Key::Named(x) => {
                    match x {
                        Named::ArrowUp => Some(Message::Up),
                        Named::ArrowDown => Some(Message::Down),
                        Named::Enter => Some(Message::Enter),
                        Named::Escape => Some(Message::Exit),
                        _ => None,
                    }
                }
                _ => None,
            }
        }

        keyboard::on_key_press(handle_hotkey)
    }


}



fn main() ->iced::Result {
    println!("Hello, world!");
    iced::application("jrip",  AppState::update, AppState::view )
        .subscription(AppState::subscription)
        .theme(|_| Theme::KanagawaDragon)
        .run()
}
