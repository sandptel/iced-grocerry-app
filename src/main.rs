use iced::widget::shader::wgpu::naga::proc::Alignment;
use iced::widget::{Column, button, column, container, row, text, text_input,scrollable};
use iced::{Element, Length, Sandbox, Settings, alignment};
use std::f32::consts::{LN_10, LOG2_10};
use std::fs::File;
use std::os::unix::raw::dev_t;
use std::{env, fs};


// the following is not inspired/ implemented from intuition from this --> https://nikolish.in/gs-with-iced-1 
pub struct GroceryList {
    value: isize,
}

#[derive(Debug, Clone)]
pub enum Message {
    Increment,
    Decrement
}

// fn view_grocery_items(input: & Vec<String>) -> Element<'static, Message>
// {
//     let mut column = Column::new();

//     for (index,i) in input.iter().enumerate()
//     {
//         let delete_button = button("delete").on_press(Message::InputRemoved(index)).padding(LN_10);
//         let text_bar = text(i);
//         let row = row![text_bar,delete_button].spacing(LOG2_10).align_items(iced::Alignment::Center);
//         column = column.push(row);  
//     }
    
//     container(column).height(Length::Fill).width(Length::Fill).into()
// }

impl Sandbox for GroceryList {
    type Message = Message;

    fn new() -> Self {
    Self{value: 0}
    }

    fn title(&self) -> String {
        "Icy Grocerry".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        match message{
            Message::Increment =>{
                self.value = self.value+1;
            },
            Message::Decrement=>{
                self.value = self.value -1;
            }
        }
    }


    fn view(&self) -> Element<'_, Self::Message> {
        // text is a method defined in Iced
        // text("Grocerries are found here").into()

        // #notes
        //this is the into trait it converts the T to U --> If a Struct T implements the
        // impl From <T> for U {} trait --> doing let var: U = T.into(); would work
        // Basically if T implements the From Trait for U then
        // U automatically implements Into trait for T

        // container(text("No Groceries )---")).height(Length::Fill).width(Length::Fill).align_x(alignment::Horizontal::Center).align_y(alignment::Vertical::Center).into()
        // if you want ot add 2 texts together mix those with column![..., ...].into() macro....
        // let groceries = vec![]

        // let mut grocerries = GroceryList::new();

        let increment = button("increment").width(Length::Fill).height(Length::Fill).on_press(Message::Increment);
        let decrement = button("decrement").width(Length::Fill).height(Length::Fill).on_press(Message::Decrement);
        let value = text(self.value).width(Length::Fill).height(Length::Fill).size(100).vertical_alignment(alignment::Vertical::Center).horizontal_alignment(alignment::Horizontal::Center);
        let row = row![decrement,value,increment].height(Length::Fill).width(Length::Fill);
        row.into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}
fn main() {
    // Force the application to use X11 instead of Wayland
    //    unsafe{ env::set_var("WINIT_UNIX_BACKEND", "x11");} --> Claude is DUMB

    let _ = GroceryList::run(Settings::default());
    // println!("{}",String::default())
}
