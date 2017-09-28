macro_rules! check {
    ($check:expr) => {{
        let result = $check;
        if ScInternalResult::Success != result {
            return match result {
                _ => Err(ErrorCode::Unhandled)
            }
        }
    }}
}

pub mod spirv;
pub mod hlsl;

mod bindings {
    #![allow(dead_code)]
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!("bindings.rs"));
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum ErrorCode {
    Unhandled = 1,
}
