use windows_reactive::{
    game::Game, hwnd_builder::create_window_handle, message_ext::dispatch_thread_events,
    pre_settings,
};

pub fn main() {
    let class_name = "NativeWindowsGuiWindow";
    pre_settings::init_window_class(class_name);

    let game = Game::initialize();

    let window = create_window_handle()
        .class_name(class_name)
        .size((500, 500))
        .position((300, 300))
        .text("Basic Window")
        .window()
        .resizable()
        .maximizable()
        .minimizable()
        .always_on_top()
        .visible()
        .build();

    dispatch_thread_events();
}
