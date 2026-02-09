use iced::{Alignment::Center, Bottom, Element, Length::Fill, Task, Theme, widget::{button, column, container, row, scrollable, text_input}};

use crate::api::auth::attemt_login;

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
    LoginResult(bool),
}

//UI MAIN
pub fn start_app() -> iced::Result {
    iced::application(App::new, App::update, App::view)
    .theme(Theme::CatppuccinMocha)
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
    fn update(&mut self, messages: Message) -> iced::Task<Message> {
        match messages {
            Message::InputCkey(ckey) => {
                self.login_form.username = ckey;
                Task::none()
            },

            Message::InputPass(pass) => {
                self.login_form.password = pass;
                Task::none()
            },

            Message::SwitchPage(page) => {
                self.page = page;
                Task::none()
            }

            Message::LoginSubmit => {
                let future = attemt_login(
                    self.login_form.username.clone(),
                    self.login_form.password.clone(),
                );

                Task::perform(future, |result| {
                    match result {
                        Ok(success) => Message::LoginResult(success),
                        Err(e) => {
                            eprintln!("Login failed: {}", e);
                            Message::LoginResult(false)
                        }
                    }
                })
            }

            Message::LoginResult(result) => {
                if result {
                    self.page = Page::Home;
                }
                Task::none()
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
                .spacing(3)
            )
            .width(Fill)
            .height(Fill)
            .into()
        };

        let home_page = ||{
            container(
                column![
                    "home_page",
                    navigation(),
                ]
                .spacing(10)
                .padding(10)
            )
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
                .spacing(10)
                .padding(10)
            )
            .into()
        };

        let settings_page = ||{
            container(
                column![
                    "settings_page",
                    navigation(),
                ]
                .spacing(10)
                .padding(10)
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
                    button("Submit").on_press(Message::LoginSubmit),
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