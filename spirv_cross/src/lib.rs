macro_rules! check {
    ($check:expr) => {{
        use ::std::ffi::CStr;
        use ::std::os::raw::c_void;

        let result = $check;
        if br::ScInternalResult::Success != result {
            if br::ScInternalResult::CompilationError == result {
                let mut message_ptr = ptr::null();

                if br::ScInternalResult::Success
                    != br::sc_internal_get_latest_exception_message(&mut message_ptr)
                {
                    return Err(ErrorCode::Unhandled);
                }

                let message = match CStr::from_ptr(message_ptr).to_owned().into_string() {
                    Err(_) => return Err(ErrorCode::Unhandled),
                    Ok(v) => v,
                };

                if br::ScInternalResult::Success
                    != br::sc_internal_free_pointer(message_ptr as *mut c_void)
                {
                    return Err(ErrorCode::Unhandled);
                }

                return Err(ErrorCode::CompilationError(message));
            }

            return Err(ErrorCode::Unhandled);
        }
    }};
}

mod compiler;

#[cfg(feature = "glsl")]
pub mod glsl;
#[cfg(feature = "hlsl")]
pub mod hlsl;
#[cfg(feature = "msl")]
pub mod msl;

pub mod spirv;

mod bindings {
    #![allow(dead_code)]
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!("bindings.rs"));
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum ErrorCode {
    Unhandled,
    CompilationError(String),
}
