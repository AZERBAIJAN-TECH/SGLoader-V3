use iced::{Alignment::Center, Element, Length::Fill, Theme, widget::{button, column, container, row, scrollable, text_input}};

//data
struct App {
    page: Page,
    login_form: Login
}

#[derive(Clone)]
//"signals"
enum Message { 
    SwitchPage(Page),
    InputCkey(String),
    InputPass(String),
    LoginSubmit,
}

//UI MAIN
pub fn start_app() -> iced::Result {
    iced::application(App::new, App::update, App::view)
    .theme(Theme::Dark)
    .window_size(iced::Size::new(640.0, 480.0))
    .title("SGLoader")
    .run()
}

impl App {
    //init state
    fn new() -> Self {
        Self { 
            page: Page::Login,
            login_form: Login { username: String::new(), password: String::new() },
        }
    }

    //how react to messages
    fn update(&mut self, messages: Message) {
        match messages {
            Message::InputCkey(ckey) => {
                self.login_form.username = ckey
            },

            Message::InputPass(pass) => {
                self.login_form.password = pass
            },

            Message::SwitchPage(page) => {
                self.page = page
            }

            Message::LoginSubmit => {
                todo!()
            }
        }
    }

    //render ui
    fn view(&self) -> Element<'_, Message> {

        let navigation = || -> Element<'_, Message>{
            container(
                row![
                    button("Home").on_press(Message::SwitchPage(Page::Home)),
                    button("Servers").on_press(Message::SwitchPage(Page::Servers)),
                    button("Settings").on_press(Message::SwitchPage(Page::Settings)),
                ]
            )
            .width(Fill)
            .height(100)
            .into()
        };

        let home_page = ||{
            container(column![
                "home_page",
                navigation(),
            ])
            .width(Fill)
            .height(Fill)
            .into()
        };
        


        let servers_page = ||{
            container(
                column![
                    container(
                        scrollable(
                            "servers_page"
                        ),
                    )
                    .width(Fill)
                    .height(Fill),

                    navigation()
            ]
        ).into()
        };

        let settings_page = ||{
            container(column![
                    "settings_page",
                    navigation(),
                ]
            )
            .width(Fill)
            .height(Fill)
            .into()
        };

        let login_page= ||{
            container(
                column![
                    text_input("ckey", &self.login_form.username)
                        .on_input(|ckey| Message::InputCkey(ckey.to_string()))
                        .width(200),
                    text_input("pass", &self.login_form.password)
                        .on_input(|pass| Message::InputPass(pass.to_string()))
                        .width(200)
                        .secure(true),
                    button("Submit").on_press(Message::SwitchPage(Page::Home)),//todo
                ]
                .align_x(Center)
                .spacing(10)
            )
            .width(Fill)
            .height(Fill)
            .align_x(Center)
            .align_y(Center)
            .into()
        };

        match self.page {
            Page::Login => login_page(),
            Page::Home => home_page(),
            Page::Servers => servers_page(),
            Page::Settings => settings_page(),
        }
    }
}

#[derive(Default, Clone)]
pub struct Login {
    username: String,
    password: String
}

#[derive(Clone)]
pub enum Page {
    Login,
    Home,
    Servers,
    Settings,
}

impl App {
    //OTHER FUNCS (DONT FORGET ABOUT PUB)
}