use iced::{Element, Sandbox};

pub struct ApplicationWindow {

}

impl Sandbox for ApplicationWindow {
    type Message = ();

    fn new() -> Self {
        Self {}
    }

    fn title(&self) -> String {
        String::from("Gaia - Minecraft Datapack Generator")
    }

    fn update(&mut self, message: Self::Message) {
        todo!()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        todo!()
    }
}