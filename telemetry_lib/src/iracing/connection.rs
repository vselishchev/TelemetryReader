use std::io::Result as ioResult;
use std::io::Error;
use std::os::windows::raw::HANDLE;
use std::ffi::c_void;
use winapi::um::memoryapi::{OpenFileMappingW, FILE_MAP_READ, MapViewOfFile, UnmapViewOfFile};
use winapi::um::handleapi::{CloseHandle};
use winapi::um::errhandlingapi::GetLastError;
use winapi::shared::minwindef::LPVOID;
use winapi::um::synchapi::OpenEventW;
use winapi::um::winnt::SYNCHRONIZE;

pub const IRSDK_MEMMAPFILENAME: &str = r"Local\IRSDKMemMapFileName";
pub const IRSDK_DATAVALIDEVENTNAME: &str = r"Local\IRSDKDataValidEvent";

/// Connection struct establishes the connection to memory mapped object.
/// Since Rust doesn't really work with MM objects, the implementation will use tons of unsafe code.
/// Additionaly, create a event object handle which which notifies when the new data is coming.
pub struct Connection {
    pub mmf_view: *mut c_void,
    mmf: HANDLE,
    pub data_event: HANDLE
}

fn retrieve_error() -> Error {
    let errno : i32;
    unsafe {
        errno = GetLastError() as i32;
    }

    return std::io::Error::from_raw_os_error(errno);
}

impl Connection {
    pub fn establish() -> ioResult<Connection> {
        let mut path: Vec<u16> = IRSDK_MEMMAPFILENAME.encode_utf16().collect();
        path.push(0);

        let mapping : HANDLE;
        unsafe {
            mapping = OpenFileMappingW(FILE_MAP_READ, 0, path.as_ptr());
        };

        if mapping.is_null() {
            return Err(retrieve_error());
        }

        let view: LPVOID;

        unsafe {
            view = MapViewOfFile(mapping, FILE_MAP_READ, 0, 0, 0);
        }

        if view.is_null() {
           return Err(retrieve_error()); 
        }

        let mut event_path: Vec<u16> = IRSDK_DATAVALIDEVENTNAME.encode_utf16().collect();
        event_path.push(0);
        let event_object : HANDLE;
        unsafe {
            event_object = OpenEventW(SYNCHRONIZE, 0, event_path.as_ptr());
        }

        if event_object.is_null() {
            return Err(retrieve_error());
        }

        Ok(Connection{mmf_view: view, mmf : mapping, data_event: event_object})
    }

    pub fn close(&self) -> ioResult<()> {
        let mut result = unsafe {
            CloseHandle(self.data_event)
        };

        if !self.mmf_view.is_null()  { 
            unsafe {
                UnmapViewOfFile(self.mmf_view);
            };
        }   

        result = result & unsafe {
            CloseHandle(self.mmf)
        };

        if result == 0  {
            Err(retrieve_error())
        } else {
            Ok(())
        }
    }
}
