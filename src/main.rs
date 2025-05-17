use iced::widget::shader::wgpu::naga::proc::Alignment;
use iced::widget::{Column, button, column, container, row, text, text_input,scrollable};
use iced::{Element, Length, Sandbox, Settings, alignment};
use std::f32::consts::{LN_10, LOG2_10};
use std::fs::File;
use std::{env, fs};

use rayon::prelude::*;
// the following is inspired/ COPIED from this --> https://www.leafheap.com/articles/iced-tutorial-version-0-12
pub struct GroceryList {
    input_value: String,
    grocery_list: Vec<String>
}

#[derive(Debug, Clone)]
pub enum Message {
    InputAdded(String),
    InputRemoved(usize),
    Submitted,
}

fn view_grocery_items(input: & Vec<String>) -> Element<'static, Message>
{
    let mut column = Column::new();

    for (index,i) in input.iter().enumerate()
    {
        let delete_button = button("delete").on_press(Message::InputRemoved(index)).padding(LN_10);
        let text_bar = text(i);
        let row = row![text_bar,delete_button].spacing(LOG2_10).align_items(iced::Alignment::Center);
        column = column.push(row);  
    }
    
    container(column).height(Length::Fill).width(Length::Fill).into()
}

impl Sandbox for GroceryList {
    type Message = Message;

    fn new() -> Self {
        Self {input_value: String::default(), grocery_list: vec!["Milk".to_owned(),"Cheese".to_owned()]}
    }

    fn title(&self) -> String {
        "Icy Grocerry".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        match message{
            Message::InputAdded(value)=>{self.input_value = value},
            Message::InputRemoved(index)=>{self.grocery_list.remove(index);},
            Message::Submitted=> {
                self.grocery_list.push(self.input_value.clone());
                self.input_value = String::default();
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
        let input_field = text_input("You have to type some Names here? ",&self.input_value).on_input(|input| Message::InputAdded(input)).on_submit(Message::Submitted);
        // let mut grocerries = GroceryList::new();
        let button = button("Submit").on_press(Message::Submitted);
        column![view_grocery_items(&self.grocery_list ),row![input_field, button]].padding(30).into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}

fn add_grocery_to_cache(input: String)
{
   let fd = File::create_new("./cache.txt");
   
}
fn main() {
    // Force the application to use X11 instead of Wayland
    //    unsafe{ env::set_var("WINIT_UNIX_BACKEND", "x11");} --> Claude is DUMB
    
    // let _ = GroceryList::run(Settings::default());
    // println!("{}",String::default())
    let vect = vec![1,2,3,4];
    let v: Vec<i32> = (1..=1000000).collect();
    // let it = vec![1,2,3,4].iter();
    // println!("{:?}",vect.into_iter());
    // vect.par_iter().for_each(|value| println!("{}",value));
    use std::time::Instant;
    use std::thread;
    use std::time::Duration;

    let start = Instant::now();
    v.iter().for_each(|value| println!("{}",value));
    let end = Instant::now();

    let duration_iter = end.duration_since(start);

    let start = Instant::now();
    v.par_iter().for_each(|value| println!("{}",value));
    let end = Instant::now();

    let duration_par = end.duration_since(start);
    
    println!("Time elapsed with par: {:?}", duration_par);

    println!("Time elapsed without par_iter: {:?}", duration_iter);
}
