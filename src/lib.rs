pub mod ffi;
pub mod methods;

use std::os::raw::c_void;

use crate::ffi::kiero::{RenderType, Status};
use crate::methods::KieroMethod;

pub fn init(render_type: RenderType) -> Status {
    ffi::kiero::init(render_type)
}

pub fn shutdown() {
    ffi::kiero::shutdown()
}

pub fn bind<M: KieroMethod>(method: M, original: *mut *mut c_void, replacement: *mut c_void) -> Status {
    unsafe { ffi::kiero::bind(method.id(), original as *mut _, replacement as *mut _) }
}

pub fn unbind<M: KieroMethod>(method: M) {
    ffi::kiero::unbind(method.id())
}

pub fn get_render_type() -> RenderType {
    ffi::kiero::get_render_type()
}

pub fn get_methods_table() -> Option<&'static [usize]> {
    let count = match get_render_type() {
        RenderType::D3D9 => methods::D3D9::METHOD_COUNT,
        RenderType::D3D10 => methods::D3D10::METHOD_COUNT,
        RenderType::D3D11 => methods::D3D11::METHOD_COUNT,
        RenderType::D3D12 => methods::D3D12::METHOD_COUNT,
        RenderType::OpenGL => methods::OpenGL::METHOD_COUNT,
        RenderType::Vulkan => methods::Vulkan::METHOD_COUNT,
        _ => return None,
    };
    Some(unsafe { std::slice::from_raw_parts(ffi::kiero::get_methods_table(), count) })
}
