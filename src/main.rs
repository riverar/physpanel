mod bindings;

use crate::bindings::*;
use std::env;
use std::ffi::c_void;
use windows::Win32::Foundation::*;
use windows::core::*;

const WNF_DX_INTERNAL_PANEL_DIMENSIONS: WNF_STATE_NAME = WNF_STATE_NAME {
    Data: [0xA3BC4875, 0x41C61629], // 0x41C61629_A3BC4875u64
};

#[derive(Debug, Clone, Copy, Default)]
struct Dimensions {
    width_mm: u32,
    height_mm: u32,
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    let action = &args[1];
    match action.as_str() {
        "get" => {
            let size = get_display_size()?;
            let diagonal_mm = ((size.width_mm.pow(2) + size.height_mm.pow(2)) as f64).sqrt();

            const MM_PER_INCH: f64 = 25.4;
            let diagonal_inches = diagonal_mm / MM_PER_INCH;

            println!(
                "Current size: {}x{}mm ({:.2}\")",
                size.width_mm, size.height_mm, diagonal_inches
            );
        }
        "set" => {
            if args.len() != 4 {
                print_usage();
            } else {
                set_display_size(Dimensions {
                    width_mm: args[2].parse().unwrap(),
                    height_mm: args[3].parse().unwrap(),
                })?;
                println!("OK.");
            }
        }
        _ => {
            print_usage();
        }
    }

    Ok(())
}

#[rustfmt::skip]
fn print_usage() {
    println!("physpanel 0.1.1");
    println!("Copyright © Rafael Rivera\n");
    println!("This program comes with ABSOLUTELY NO WARRANTY.");
    println!("This is free software, and you are welcome to redistribute it under certain conditions.\n");
    println!("Usage: physpanel <get|set> [width_mm height_mm]");
}

fn get_display_size() -> Result<Dimensions> {
    let mut change_stamp = 0u32;
    let mut output = Dimensions::default();

    let status = unsafe {
        RtlQueryWnfStateData(
            &mut change_stamp,
            WNF_DX_INTERNAL_PANEL_DIMENSIONS,
            Some(query_callback),
            Some(&mut output as *mut _ as *const c_void),
            None,
        )
    };

    status.ok().map(|_| output)
}

fn set_display_size(dims: Dimensions) -> Result<()> {
    let dimensions: u64 = ((dims.height_mm as u64) << 32) | (dims.width_mm as u64);

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
