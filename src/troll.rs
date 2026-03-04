use rand::Rng;
use std::thread;
use std::time::Duration;

use windows_sys::Win32::UI::Input::KeyboardAndMouse::{MOUSEEVENTF_MOVE, mouse_event};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, SC_MONITORPOWER, SendMessageW, WM_SYSCOMMAND,
};

pub fn troll_script() {
    let mut rng = rand::thread_rng();

    loop {
        let wait_time = rng.gen_range(10..60);
        thread::sleep(Duration::from_secs(wait_time));

        unsafe {
            let hwnd = GetForegroundWindow();
            if hwnd != 0 {
                SendMessageW(hwnd, WM_SYSCOMMAND, SC_MONITORPOWER as usize, 2);

                let glitch_duration = rng.gen_range(30..90);
                thread::sleep(Duration::from_secs(glitch_duration));

                mouse_event(MOUSEEVENTF_MOVE, 0, 1, 0, 0);
                mouse_event(MOUSEEVENTF_MOVE, 0, -1, 0, 0);
            }
        }
    }
}
