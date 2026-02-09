mod ui;
mod api;
//mod loader;

pub const AUTH_URL:&str = "https://auth.spacestation14.com";
pub const HUB_URL:&str = "";

fn main() {
    let _ = ui::ui::start_app();
}
