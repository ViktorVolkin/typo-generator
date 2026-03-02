use std::sync::Mutex;
use std::time::{Duration, Instant};
use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::*;
use windows_sys::Win32::UI::WindowsAndMessaging::*;

const WH_KEYBOARD_LL: i32 = 13;
const WM_KEYDOWN: u32 = 0x0100;
const LLKHF_INJECTED: u32 = 0x00000010;
const VK_BACK: u16 = 0x08;

static LAST_TIME: Mutex<Option<Instant>> = Mutex::new(None);
static LAST_KEY: Mutex<Option<u32>> = Mutex::new(None);

fn send_input(vk_code: u16) {
    unsafe {
        let mut input: INPUT = std::mem::zeroed();
        input.r#type = INPUT_KEYBOARD;
        input.Anonymous.ki.wVk = vk_code;

        SendInput(1, &input, std::mem::size_of::<INPUT>() as i32);

        input.Anonymous.ki.dwFlags = KEYEVENTF_KEYUP;
        SendInput(1, &input, std::mem::size_of::<INPUT>() as i32);
    }
}

unsafe extern "system" fn low_level_handler(
    code: i32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    if code >= 0 && w_param as u32 == WM_KEYDOWN {
        let kbd = *(l_param as *const KBDLLHOOKSTRUCT);

        if (kbd.flags & LLKHF_INJECTED) != 0 {
            return CallNextHookEx(0, code, w_param, l_param);
        }

        let mut last_k_guard = LAST_KEY.lock().unwrap();
        let mut last_t_guard = LAST_TIME.lock().unwrap();
        let now = Instant::now();

        if let (Some(prev_k), Some(prev_t)) = (*last_k_guard, *last_t_guard) {
            let elapsed = now.duration_since(prev_t);

            if elapsed < Duration::from_millis(100) {
                send_input(VK_BACK);
                send_input(kbd.vkCode as u16);
                send_input(prev_k as u16);

                *last_k_guard = None;
                *last_t_guard = Some(now);

                return 1;
            }
        }

        *last_k_guard = Some(kbd.vkCode);
        *last_t_guard = Some(now);
    }

    CallNextHookEx(0, code, w_param, l_param)
}

pub fn troll_script() {
    unsafe {
        let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(low_level_handler), 0, 0);

        if hook != 0 {
            let mut msg: MSG = std::mem::zeroed();
            while GetMessageW(&mut msg, 0, 0, 0) > 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            UnhookWindowsHookEx(hook);
        }
    }
}
