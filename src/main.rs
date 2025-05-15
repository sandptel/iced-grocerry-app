use iced::{Element, Sandbox, Settings};
use iced::widget::text;
use std::env;

pub struct GroceryList
{

}

#[derive(Debug,Clone)]
pub enum Message
{

}

impl Sandbox for GroceryList
{
    type Message = Message;

    fn new() -> Self {
        Self{}
    }

    fn title(&self) -> String {
       "Icy Grocerry".to_string() 
    }

    fn update(&mut self, message: Self::Message) {
        
    }

    fn view(&self) -> Element<'_, Self::Message> {
            // text is a method defined in Iced
            text("Grocerries are found here").into()
            
            // #notes
            //this is the into trait it converts the T to U --> If a Struct T implements the 
            // impl From <T> for U {} trait --> doing let var: U = T.into(); would work 
            // Basically if T implements the From Trait for U then  
            // U automatically implements Into trait for T
    }
}

fn main() {
    // Force the application to use X11 instead of Wayland
//    unsafe{ env::set_var("WINIT_UNIX_BACKEND", "x11");}

    let _ =  GroceryList::run(Settings::default());
}


