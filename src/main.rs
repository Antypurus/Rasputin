mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

extern crate glfw;

use glfw::{Action, Context, Key};

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    let (mut window, events) = glfw
        .create_window(1280, 720, "Rasputin Renderer", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);

    while !window.should_close() {
        glfw.poll_events();
    }

    println!("Hello, world!");
}
