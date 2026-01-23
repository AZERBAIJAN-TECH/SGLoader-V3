mod ui;
mod network;
//mod loader;

pub const AUTH_URL:&str = "https://auth.spacestation14.com/";

fn main() {
    let _ = ui::ui::start_app();
}
