#[cfg(windows)]
extern crate winapi;


/// A helper function to get a Pid from the name of a process
#[cfg(windows)]
pub fn get_pid(process_name: &str) -> process_memory::Pid {
    /// A helper function to turn a c_char array to a String
    fn utf8_to_string(bytes: &[i8]) -> String {
        use std::ffi::CStr;
        unsafe {
            CStr::from_ptr(bytes.as_ptr())
                .to_string_lossy()
                .into_owned()
        }
    }
    let mut entry = winapi::um::tlhelp32::PROCESSENTRY32 {
        dwSize: std::mem::size_of::<winapi::um::tlhelp32::PROCESSENTRY32>() as u32,
        cntUsage: 0,
        th32ProcessID: 0,
        th32DefaultHeapID: 0,
        th32ModuleID: 0,
        cntThreads: 0,
        th32ParentProcessID: 0,
        pcPriClassBase: 0,
        dwFlags: 0,
        szExeFile: [0; winapi::shared::minwindef::MAX_PATH],
    };
    let snapshot: winapi::um::winnt::HANDLE;
    unsafe {
        snapshot = winapi::um::tlhelp32::CreateToolhelp32Snapshot(
            winapi::um::tlhelp32::TH32CS_SNAPPROCESS,
            0,
        );
        if winapi::um::tlhelp32::Process32First(snapshot, &mut entry)
            == winapi::shared::minwindef::TRUE
        {
            while winapi::um::tlhelp32::Process32Next(snapshot, &mut entry)
                == winapi::shared::minwindef::TRUE
            {
                if utf8_to_string(&entry.szExeFile) == process_name {
                    return entry.th32ProcessID;
                }
            }
        }
    }
    0
}

use std::ptr::*;
#[cfg(windows)]
use winapi::ctypes::c_void;
use winapi::shared::minwindef::{HMODULE, LPDWORD};
use winapi::um::processthreadsapi::*;
use winapi::um::psapi::*;
use winapi::um::winnt::*;
pub fn list_modules(process_id: winapi::shared::minwindef::DWORD) -> std::io::Result<()> {
    unsafe {
        let h_process: HANDLE =
            OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, process_id);
        if h_process != null_mut() {
            let modules: [HMODULE; 1024] = [0 as HMODULE; 1024];
            let mut cb_needed: u32 = 0u32;
            if EnumProcessModules(
                h_process,
                modules.as_ptr() as *mut HMODULE,
                std::mem::size_of::<HMODULE>() as u32 * 1024,
                &mut cb_needed as *mut _ as *mut u32,
            ) == 0
            {
                println!(
                    "enum process modules failed {}",
                    std::io::Error::last_os_error()
                );
                return Ok(());
            }
            println!("cb_needed = {:?}", cb_needed);
            let mut i: usize = 0;
            while (i as u32) < cb_needed {
                let module = modules[i];

                let mut process_name: Vec<u16> = Vec::new();
                let mut count = 0;
                while count < 40 {
                    process_name.push(0u16);
                    count += 1;
                }
                GetModuleBaseNameW(h_process, module, process_name.as_ptr() as *mut u16, 40);

                count = 0;

                while process_name[count as usize] != 0 {
                    count += 1;
                }
                let mut process_name_string: String = String::new();
                process_name_string = String::from_utf16(&process_name[..count as usize]).unwrap();
                println!(
                    "module = {:?}, process name = {}",
                    module, process_name_string
                );
                i += 1;
            }
        }
    }
    Ok(())
}

pub fn get_module_handle(
    process_id: winapi::shared::minwindef::DWORD,
    name: String,
) -> std::result::Result<HMODULE, String> {
    unsafe {
        let h_process: HANDLE =
            OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, process_id);
        if h_process != null_mut() {
            let modules: [HMODULE; 1024] = [0 as HMODULE; 1024];
            let mut cb_needed: u32 = 0u32;
            if EnumProcessModules(
                h_process,
                modules.as_ptr() as *mut HMODULE,
                std::mem::size_of::<HMODULE>() as u32 * 1024,
                &mut cb_needed as *mut _ as *mut u32,
            ) == 0
            {
                let ret = format!(
                    "enum process modules failed {}",
                    std::io::Error::last_os_error()
                );
                return Err(ret);
            }
            // println!("cb_needed = {:?}", cb_needed);
            let mut i: usize = 0;
            while (i as u32) < cb_needed {
                let module = modules[i];

                let mut process_name: Vec<u16> = Vec::new();
                let mut count = 0;
                while count < 40 {
                    process_name.push(0u16);
                    count += 1;
                }
                GetModuleBaseNameW(h_process, module, process_name.as_ptr() as *mut u16, 40);

                count = 0;

                while process_name[count as usize] != 0 {
                    count += 1;
                }
                if String::from_utf16(&process_name[..count as usize]).unwrap() == name {
                    return Ok(module);
                }
                // println!("module = {:?}, process name = {}", module, process_name);
                i += 1;
            }
        }
    }
    Err(String::from("failed"))
}

pub fn get_offset(
    process_id: winapi::shared::minwindef::DWORD,
) -> std::result::Result<usize, String> {
    unsafe {
        let process: HANDLE =
            OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, process_id);
        if process != null_mut() {
            let mut h_module: HMODULE = null_mut();
            let mut cb_needed: u32 = 0u32;
            if EnumProcessModules(
                process,
                &mut h_module,
                std::mem::size_of::<HMODULE>() as u32,
                &mut cb_needed as *mut _ as *mut u32,
            ) == 0
            {
                return Err(String::from("failed"));
            }
            return Ok(h_module as usize);
        }
    }
    Err(String::from("failed"))
}

pub fn alloc_and_write_memory(
    process_id: winapi::shared::minwindef::DWORD,
    num_bytes: usize,
    bytes: &Vec<u8>,
) -> std::result::Result<*mut c_void, String> {
    unsafe {
        let process: HANDLE = OpenProcess(PROCESS_ALL_ACCESS, 0, process_id);
        if process != null_mut() {
            let mem_ptr = winapi::um::memoryapi::VirtualAllocEx(
                process,
                null_mut(),
                num_bytes,
                winapi::um::winnt::MEM_COMMIT,
                winapi::um::winnt::PAGE_READWRITE,
            );
            println!("mem_ptr is {:?}", mem_ptr);
            let mut bytes_written: winapi::shared::basetsd::SIZE_T = 0;
            let mut bytes_written_sum = 0;

            let mut bytes_left = num_bytes;
            const BUFFER_SIZE: usize = 1024;
            while bytes_left > BUFFER_SIZE {
                let mut buffer: [u8; BUFFER_SIZE] = [0 as u8; BUFFER_SIZE];

                for i in bytes_written_sum..(bytes_written_sum + BUFFER_SIZE) {
                    buffer[i - bytes_written_sum] = bytes[i];
                }

                let address = (mem_ptr as u64 + bytes_written_sum as u64) as *mut c_void;
                println!("address is {:?}", address);
                let result = winapi::um::memoryapi::WriteProcessMemory(
                    process,
                    address,
                    buffer.as_ptr() as *mut winapi::ctypes::c_void,
                    BUFFER_SIZE,
                    &mut bytes_written as *mut winapi::shared::basetsd::SIZE_T,
                );
                if result == 0 {
                    println!("write in loop failed {}", std::io::Error::last_os_error());
                    return Err(String::from("failed write in loop"));
                }
                bytes_written_sum += bytes_written;
                bytes_written = 0;
                bytes_left -= BUFFER_SIZE;
            }

            let mut buffer: [u8; BUFFER_SIZE] = [0 as u8; BUFFER_SIZE];

            for i in bytes_written_sum..(num_bytes) {
                buffer[i - bytes_written_sum] = bytes[i];
            }

            let address = (mem_ptr as u64 + bytes_written_sum as u64) as *mut c_void;
            println!("address is {:?}", address);
            let result = winapi::um::memoryapi::WriteProcessMemory(
                process,
                address,
                buffer.as_ptr() as *mut winapi::ctypes::c_void,
                bytes_left,
                &mut bytes_written as *mut winapi::shared::basetsd::SIZE_T,
            );
            if result == 0 {
                println!(
                    "WriteProcessMemory failed {}",
                    std::io::Error::last_os_error()
                );
                return Err(String::from("failed write after loop"));
            }
            bytes_written_sum += bytes_written;
            println!("wrote {} bytes", bytes_written_sum);

            return Ok(mem_ptr);
        }
    }

    Err(String::from("failed on the outside"))
}

pub fn alloc_and_write_memory_handle(
    process: *mut winapi::ctypes::c_void,
    num_bytes: usize,
    bytes: &Vec<u8>,
) -> std::result::Result<*mut c_void, String> {
    unsafe {
        if process != null_mut() {
            let mem_ptr = winapi::um::memoryapi::VirtualAllocEx(
                process,
                null_mut(),
                num_bytes,
                winapi::um::winnt::MEM_COMMIT,
                winapi::um::winnt::PAGE_READWRITE,
            );
            println!("mem_ptr is {:?}", mem_ptr);
            let mut bytes_written: winapi::shared::basetsd::SIZE_T = 0;
            let mut bytes_written_sum = 0;

            let mut bytes_left = num_bytes;
            const BUFFER_SIZE: usize = 1024;
            while bytes_left > BUFFER_SIZE {
                let mut buffer: [u8; BUFFER_SIZE] = [0 as u8; BUFFER_SIZE];

                for i in bytes_written_sum..(bytes_written_sum + BUFFER_SIZE) {
                    buffer[i - bytes_written_sum] = bytes[i];
                }

                let address = (mem_ptr as u64 + bytes_written_sum as u64) as *mut c_void;
                println!("address is {:?}", address);
                let result = winapi::um::memoryapi::WriteProcessMemory(
                    process,
                    address,
                    buffer.as_ptr() as *mut winapi::ctypes::c_void,
                    BUFFER_SIZE,
                    &mut bytes_written as *mut winapi::shared::basetsd::SIZE_T,
                );
                if result == 0 {
                    println!("write in loop failed {}", std::io::Error::last_os_error());
                    return Err(String::from("failed write in loop"));
                }
                bytes_written_sum += bytes_written;
                bytes_written = 0;
                bytes_left -= BUFFER_SIZE;
            }

            let mut buffer: [u8; BUFFER_SIZE] = [0 as u8; BUFFER_SIZE];

            for i in bytes_written_sum..(num_bytes) {
                buffer[i - bytes_written_sum] = bytes[i];
            }

            let address = (mem_ptr as u64 + bytes_written_sum as u64) as *mut c_void;
            println!("address is {:?}", address);
            let result = winapi::um::memoryapi::WriteProcessMemory(
                process,
                address,
                buffer.as_ptr() as *mut winapi::ctypes::c_void,
                bytes_left,
                &mut bytes_written as *mut winapi::shared::basetsd::SIZE_T,
            );
            if result == 0 {
                println!(
                    "WriteProcessMemory failed {}",
                    std::io::Error::last_os_error()
                );
                return Err(String::from("failed write after loop"));
            }
            bytes_written_sum += bytes_written;
            println!("wrote {} bytes", bytes_written_sum);

            return Ok(mem_ptr);
        }
    }

    Err(String::from("failed on the outside"))
}

// use winapi::um::processthreadsapi::*;
// use winapi::um::psapi::*;
#[cfg(windows)]
// use winapi::um::winnt::*;
// use winapi::ctypes::{c_void};
// use winapi::shared::minwindef::{LPDWORD, HMODULE};
// use std::ptr;
pub fn print_process_name_and_id(process_id: u32) -> String {
    unsafe {
        let mut process_name_string: String = String::new();
        let process: HANDLE =
            OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, process_id);
        if process != null_mut() {
            let mut h_module: HMODULE = null_mut();
            let mut cb_needed: u32 = 0u32;
            if EnumProcessModules(
                process,
                &mut h_module,
                std::mem::size_of::<HMODULE>() as u32,
                &mut cb_needed as *mut _ as *mut u32,
            ) == 0
            {
                return String::new();
            }
            let mut process_name: Vec<u16> = Vec::new();
            let mut count = 0;
            while count < 40 {
                process_name.push(0u16);
                count += 1;
            }
            GetModuleBaseNameW(process, h_module, process_name.as_ptr() as *mut u16, 40);

            count = 0;

            while process_name[count as usize] != 0 {
                count += 1;
            }
            process_name_string = String::from_utf16(&process_name[..count as usize]).unwrap();
        }
        return process_name_string;
    }
}