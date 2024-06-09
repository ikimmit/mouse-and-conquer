use mouse_rs::{types::keys::Keys, Mouse};

use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};
use winit::event_loop::{ControlFlow, EventLoop};

#[cfg(target_os = "macos")]
use core_graphics::display::CGDisplay;

#[cfg(target_os = "macos")]
pub fn current_resolution() -> (i32, i32) {
    let display = CGDisplay::main();
    let width = display.pixels_wide();
    let height = display.pixels_high();
    (width as i32, height as i32)
}

fn key_released(id: u32, event: &GlobalHotKeyEvent) -> bool {
    id == event.id && event.state == HotKeyState::Released
    // id == event.id && event.state == HotKeyState::Pressed
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let hotkeys_manager = GlobalHotKeyManager::new().unwrap();

    let hotkey_activate = HotKey::new(Some(Modifiers::META), Code::KeyH);
    let hotkey_west = HotKey::new(None, Code::KeyS);
    let hotkey_east = HotKey::new(None, Code::KeyF);
    let hotkey_north = HotKey::new(None, Code::KeyE);
    let hotkey_south = HotKey::new(None, Code::KeyD);
    let hotkey_click1 = HotKey::new(None, Code::KeyH);

    hotkeys_manager.register(hotkey_activate).unwrap();

    let global_hotkey_channel = GlobalHotKeyEvent::receiver();
    let mut is_active = false;
    let mut f_w = 0;
    let mut f_h = 0;
    let mut m_w = 0;
    let mut m_h = 0;
    let mouse = Mouse::new();
    let (w, h) = current_resolution();

    event_loop
        .run(move |_event, event_loop| {
            event_loop.set_control_flow(ControlFlow::Poll);

            if let Ok(event) = global_hotkey_channel.try_recv() {
                println!("{event:?}");

                if key_released(hotkey_activate.id(), &event) {
                    f_w = w / 2;
                    f_h = h / 2;
                    m_w = f_w;
                    m_h = f_h;

                    println!("Mv {} {} {} {}", m_w, m_h, f_w, f_h);
                    mouse.move_to(m_w, m_h).expect("No move mouse");

                    if !is_active {
                        hotkeys_manager.register(hotkey_west).unwrap();
                        hotkeys_manager.register(hotkey_east).unwrap();
                        hotkeys_manager.register(hotkey_north).unwrap();
                        hotkeys_manager.register(hotkey_south).unwrap();
                        hotkeys_manager.register(hotkey_click1).unwrap();
                    }

                    is_active = true;
                } else if key_released(hotkey_click1.id(), &event) {
                    hotkeys_manager.unregister(hotkey_west).unwrap();
                    hotkeys_manager.unregister(hotkey_east).unwrap();
                    hotkeys_manager.unregister(hotkey_north).unwrap();
                    hotkeys_manager.unregister(hotkey_south).unwrap();
                    hotkeys_manager.unregister(hotkey_click1).unwrap();

                    println!("Click {} {} {} {}", m_w, m_h, f_w, f_h);
                    mouse.press(&Keys::LEFT).expect("No press button");
                    mouse.release(&Keys::LEFT).expect("No release button");

                    is_active = false;
                } else if key_released(hotkey_west.id(), &event) {
                    f_w = std::cmp::max(f_w / 2, 5);
                    m_w -= f_w;
                    println!("WE {} {} {} {}", m_w, m_h, f_w, f_h);
                    mouse.move_to(m_w, m_h).expect("No move mouse");
                } else if key_released(hotkey_east.id(), &event) {
                    f_w = std::cmp::max(f_w / 2, 5);
                    m_w += f_w;
                    println!("EA {} {} {} {}", m_w, m_h, f_w, f_h);
                    mouse.move_to(m_w, m_h).expect("No move mouse");
                } else if key_released(hotkey_north.id(), &event) {
                    f_h = std::cmp::max(f_h / 2, 5);
                    m_h -= f_h;
                    println!("WE {} {} {} {}", m_w, m_h, f_w, f_h);
                    mouse.move_to(m_w, m_h).expect("No move mouse");
                } else if key_released(hotkey_south.id(), &event) {
                    f_h = std::cmp::max(f_h / 2, 5);
                    m_h += f_h;
                    println!("EA {} {} {} {}", m_w, m_h, f_w, f_h);
                    mouse.move_to(m_w, m_h).expect("No move mouse");
                }
            }
        })
        .unwrap();
}
