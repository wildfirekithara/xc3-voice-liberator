use std::ffi::CStr;
use std::ffi::CString;

use skyline::{hook, from_offset};

/// Alias for println!, but disabled on release profiles
macro_rules! dbg_println {
    ($($arg:tt)*) => {{
        #[cfg(debug_assertions)]
        println!($($arg)*);
    }};
}

// WWiseLoadPack
#[from_offset(0x0060b34)]
fn wwise_load_pack(wwise: u64, p2: u32, filename: *const u8, p4: u64) -> u64;

// WWiseLoadBank2
#[from_offset(0x0061440)]
fn wwise_load_bank2(wwise: u64, p2: u32, filename: *const u8, p4: u64, p5: u64) -> u64;

fn load_vo_bank(wwise: u64, id: i32) -> u64 {
    let filename = format!("vo_{:03}.bnk", id);
    let filename_cstr: CString = CString::new(filename).unwrap();
    let filename_cstr: &CStr = filename_cstr.as_c_str();

    dbg_println!("[XC3-Voice-Liberator] Loading {}", std::str::from_utf8(filename).unwrap());
    let res: u64;

    unsafe {
        res = wwise_load_bank2(wwise, 0, filename_cstr.as_ptr() as *const u8, 0, 0);
    }
    return res
}

fn load_dlc_pack(wwise: u64, id: i32, lang: u64) -> u64 {
    let lang = match lang & 1 {
        0 => "en",
        _ => "jp",
    };

    let filename = format!("{}_dlc{:}.pck", lang, id);
    let filename_cstr: CString = CString::new(filename).unwrap();
    let filename_cstr: &CStr = filename_cstr.as_c_str();

    dbg_println!("[XC3-Voice-Liberator] Loading {}", std::str::from_utf8(filename).unwrap());
    let res: u64;

    unsafe {
        res = wwise_load_pack(wwise, 0, filename_cstr.as_ptr() as *const u8, 0);
    }
    return res
}

// WWiseLoadBaseGame
#[hook(offset = 0x00bfc174)]
unsafe fn wwise_load_base_game(wwise: u64, has_dlc2: u64, has_dlc3: u64, has_dlc4: u64, lang: u64) -> u64 {
    let mut res = call_original!(wwise, has_dlc2, has_dlc3, has_dlc4, lang);

    if (has_dlc4 & 1) != 0 {
        dbg_println!("[XC3-Voice-Liberator] Loading Future Redeemed VO");

        for i in 0x24..0x2C { // Matthew to Na'el / Alpha
            res = load_vo_bank(wwise, i);
        }
    }

    return res;
}

// WWiseLoadFutureRedeemed
#[hook(offset = 0x00bfbed4)]
unsafe fn wwise_load_future_redeemed(wwise: u64, lang: u64) -> u64 {
    call_original!(wwise, lang);

    dbg_println!("[XC3-Voice-Liberator] Loading Base Game VO");
    load_dlc_pack(wwise, 2, lang); // needed for Ino
    load_dlc_pack(wwise, 3, lang); // needed for Masha
    let mut res: u64 = 0;

    for i in 0x01..0x24 { // Noah to Hero Rex
        if i == 0x0E || i == 0x1C { // Skip Joran and Nimue
            continue;
        }
        res = load_vo_bank(wwise, i);
    }

    return res;
}

#[skyline::main(name = "xc3_voice_liberator")]
pub fn main() {
    dbg_println!("[XC3-Voice-Liberator] Loading...");
    skyline::install_hooks!(wwise_load_base_game, wwise_load_future_redeemed);
    dbg_println!("[XC3-Voice-Liberator] Loaded!");
}
