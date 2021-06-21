use algonaut::core::Address;
use core_foundation::{
    base::TCFType,
    string::{CFString, CFStringRef, __CFString},
};
use libc::c_char;

use crate::{bootstrap, get_infos, provider::AccountViewData};

enum FFIStatus {
    Success = 1,
    // Refine this in a real project
    Error = 2,
}

#[no_mangle]
pub unsafe extern "C" fn ffi_bootstrap() -> FFIVoidResult {
    match bootstrap() {
        Ok(_) => FFIVoidResult {
            status: FFIStatus::Success as i32,
            error: "".to_cfstringref_and_forget(),
        },
        Err(e) => FFIVoidResult {
            status: FFIStatus::Error as i32,
            error: format!("Couldn't bootstrap: {}", e).to_cfstringref_and_forget(),
        },
    }
}

#[no_mangle]
pub unsafe extern "C" fn ffi_get_infos(address: *const c_char) -> FFIGetInfosResult {
    let adr_str = cstring_to_str(&address);
    match adr_str.parse::<Address>() {
        Ok(address) => match get_infos(&address) {
            Ok(view_data) => FFIGetInfosResult {
                status: FFIStatus::Success as i32,
                error: "".to_cfstringref_and_forget(),
                data: to_ffi_account_view_data(&view_data),
            },
            Err(e) => FFIGetInfosResult {
                status: FFIStatus::Error as i32,
                error: format!("Couldn't get infos: {}", e).to_cfstringref_and_forget(),
                data: to_ffi_account_view_data(&AccountViewData::default()),
            },
        },
        Err(e) => FFIGetInfosResult {
            status: FFIStatus::Error as i32,
            error: format!("Couldn't parse address: {}", e).to_cfstringref_and_forget(),
            data: to_ffi_account_view_data(&AccountViewData::default()),
        },
    }
}

unsafe fn cstring_to_str<'a>(cstring: &'a *const c_char) -> &str {
    if cstring.is_null() {
        // Return Result in a real project
        panic!("cstring is null")
    }
    let raw = ::std::ffi::CStr::from_ptr(*cstring);
    raw.to_str().expect("Couldn't convert c string to slice")
}

unsafe fn to_ffi_account_view_data(view_data: &AccountViewData) -> FFIAccountViewData {
    FFIAccountViewData {
        address: view_data.address.to_cfstringref_and_forget(),
        status: view_data.status.to_cfstringref_and_forget(),
        holdings: view_data.holdings.to_cfstringref_and_forget(),
        rewards: view_data.rewards.to_cfstringref_and_forget(),
        pending_rewards: view_data.pending_rewards.to_cfstringref_and_forget(),
    }
}

// For simplicity, we return a generic result with a JSON payload
// You can refine this to return the specific structs being requested, to skip serialization
#[repr(C)]
#[derive(Debug)]
pub struct FFIVoidResult {
    pub status: i32,
    pub error: CFStringRef,
}

#[repr(C)]
#[derive(Debug)]
pub struct FFIGetInfosResult {
    pub status: i32,
    pub error: CFStringRef,
    pub data: FFIAccountViewData,
}

#[repr(C)]
#[derive(Debug)]
pub struct FFIAccountViewData {
    pub address: CFStringRef,
    pub status: CFStringRef,
    pub holdings: CFStringRef,
    pub rewards: CFStringRef,
    pub pending_rewards: CFStringRef,
}

trait StringFFIAdditions {
    fn to_cfstringref_and_forget(self) -> *const __CFString;
}

impl StringFFIAdditions for &str {
    fn to_cfstringref_and_forget(self) -> *const __CFString {
        let session_cf_string = CFString::new(self);
        let cf_string_ref = session_cf_string.as_concrete_TypeRef();
        ::std::mem::forget(session_cf_string);
        cf_string_ref
    }
}
