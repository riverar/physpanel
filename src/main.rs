mod bindings;

use crate::bindings::*;
use std::env;
use std::ffi::c_void;
use windows::Win32::Foundation::*;
use windows::core::*;

const WNF_DX_INTERNAL_PANEL_DIMENSIONS: WNF_STATE_NAME = WNF_STATE_NAME {
    Data: [0xA3BC4875, 0x41C61629], // 0x41C61629_A3BC4875u64
};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    let action = &args[1];
    if action == "get" {
        let size = get_display_size()?;
        let width = size.0;
        let height = size.1;
        let diagonal_mm = ((width * width + height * height) as f64).sqrt();
        let diagonal_inches = diagonal_mm / 25.4;
        println!(
            "Current size: {}x{}mm ({:.2}\")",
            width, height, diagonal_inches
        );
    } else if action == "set" {
        if args.len() != 4 {
            print_usage();
        } else {
            set_display_size(args[2].parse().unwrap(), args[3].parse().unwrap())?;
            println!("OK.");
        }
    } else {
        print_usage();
    }

    Ok(())
}

fn print_usage() {
    println!("physpanel 0.1.0");
    println!("Copyright © Rafael Rivera\n");
    println!("This program comes with ABSOLUTELY NO WARRANTY.");
    println!("This is free software, and you are welcome to redistribute it under certain conditions.\n");
    println!("Usage: physpanel <get|set> [width_mm height_mm]");
}

fn get_display_size() -> Result<(u32, u32)> {
    let mut change_stamp = 0u32;
    let mut output = (0u32, 0u32);

    let status = unsafe {
        RtlQueryWnfStateData(
            &mut change_stamp,
            WNF_DX_INTERNAL_PANEL_DIMENSIONS,
            Some(query_callback),
            Some(&mut output as *mut _ as *const c_void),
            None,
        )
    };

    if status == STATUS_SUCCESS {
        Ok(output)
    } else {
        Err(Error::from(status))
    }
}

fn set_display_size(width_mm: u32, height_mm: u32) -> Result<()> {
    let dimensions: u64 = ((height_mm as u64) << 32) | (width_mm as u64);

    let status = unsafe {
        RtlPublishWnfStateData(
            WNF_DX_INTERNAL_PANEL_DIMENSIONS,
            None,
            Some(&dimensions as *const _ as *const c_void),
            Some(8),
            None,
        )
    };

    status.ok()
}

unsafe extern "system" fn query_callback(
    _state_name: WNF_STATE_NAME,
    _change_stamp: u32,
    _type_id: *const WNF_TYPE_ID,
    callback_context: *const c_void,
    buffer: *const c_void,
    buffer_size: u32,
) -> NTSTATUS {
    if !buffer.is_null() && buffer_size == 8 {
        unsafe {
            let dimensions = *(buffer as *const u64);
            let width = (dimensions & 0xFFFFFFFF) as u32;
            let height = ((dimensions >> 32) & 0xFFFFFFFF) as u32;

            if !callback_context.is_null() {
                let output = callback_context as *mut (u32, u32);
                *output = (width, height);
            }
        }
    }

    STATUS_SUCCESS
}