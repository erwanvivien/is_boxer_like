use is_boxer_like::App;
use windows::vk;

fn main() -> Result<(), eframe::Error> {
    let mut app = App::new();
    app.update_windows("warcraft");
    std::thread::sleep(std::time::Duration::from_secs(1));
    app.layout_windows();
    loop {
        app.send_key(is_boxer_like::WPARAM(vk::VK_F1.0 as usize));
    }
}
