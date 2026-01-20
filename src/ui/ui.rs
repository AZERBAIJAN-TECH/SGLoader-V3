use iced::{Element, Theme, widget::{button::text, container, row}};

//data
struct App { 

}

//"signals"
struct Message { 

}

//UI MAIN
pub(crate) fn start_app() -> iced::Result {
    iced::application(App::new, App::update, App::view)
    .theme(Theme::Dark)
    .window_size(iced::Size::new(640.0, 480.0))
    .run()
}

impl App {
    //init state
    fn new() -> Self {
        Self {  }
    }

    //how react to messages
    fn update(&mut self, messages: Message) {

    }

    //render ui
    fn view(&self) -> Element<'_, Message> {

        container(row![]).into()

    }
}

impl App {
    //OTHER FUNCS (DONT FORGET ABOUT PUB)
}

