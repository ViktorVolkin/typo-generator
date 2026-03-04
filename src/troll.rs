use rand::Rng;
use std::thread;
use std::time::Duration;

use windows_sys::Win32::UI::Input::KeyboardAndMouse::{MOUSEEVENTF_MOVE, mouse_event};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, MB_ICONERROR, MB_OK, MessageBoxW, SC_MONITORPOWER, SendMessageW,
    WM_SYSCOMMAND,
};

fn to_utf16(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

pub fn troll_script() {
    let mut rng = rand::thread_rng();

    loop {
        // Ждем от 1 до 7 минут
        let wait_time = rng.gen_range(60..420);
        thread::sleep(Duration::from_secs(wait_time));

        unsafe {
            let hwnd = GetForegroundWindow();
            if hwnd != 0 {
                // 1. Гасим монитор
                SendMessageW(hwnd, WM_SYSCOMMAND, SC_MONITORPOWER as usize, 2);

                // 2. Ждем "глюк" (1-5 сек)
                let glitch_duration = rng.gen_range(1..5);
                thread::sleep(Duration::from_secs(glitch_duration));

                // 3. Будим монитор
                mouse_event(MOUSEEVENTF_MOVE, 0, 1, 0, 0);
                mouse_event(MOUSEEVENTF_MOVE, 0, -1, 0, 0);

                // 4. СРАЗУ выбрасываем ошибку GPU
                // Даем системе полсекунды "прийти в себя" перед окном
                thread::sleep(Duration::from_millis(500));

                let title = to_utf16("System Error - GPU Display Driver");
                let message = to_utf16(
                    "Display driver 'nvlddmkm' stopped responding and has successfully recovered. Hardware acceleration disabled for stability.",
                );

                MessageBoxW(
                    0,
                    message.as_ptr(),
                    title.as_ptr(),
                    MB_OK | MB_ICONERROR, // Кнопка ОК и иконка красного крестика
                );
            }
        }
    }
}
