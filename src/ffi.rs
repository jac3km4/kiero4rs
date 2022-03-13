use cxx::{kind, type_id, ExternType};

#[derive(Debug)]
#[repr(i32)]
pub enum Status {
    UnknownError = -1,
    NotSupportedError = -2,
    ModuleNotFoundError = -3,

    AlreadyInitializedError = -4,
    NotInitializedError = -5,

    Success = 0,
}

#[derive(Debug)]
#[repr(i32)]
pub enum RenderType {
    None,

    D3D9,
    D3D10,
    D3D11,
    D3D12,

    OpenGL,
    Vulkan,

    Auto,
}

unsafe impl ExternType for Status {
    type Id = type_id!("glue::Status");
    type Kind = kind::Trivial;
}

unsafe impl ExternType for RenderType {
    type Id = type_id!("glue::RenderType");
    type Kind = kind::Trivial;
}

#[cxx::bridge(namespace = "kiero")]
pub mod kiero {

    #[namespace = "glue"]
    unsafe extern "C++" {
        include!("glue.h");

        type CVoid;
        #[namespace = "glue"]
        type Status = super::Status;
        #[namespace = "glue"]
        type RenderType = super::RenderType;
    }

    unsafe extern "C++" {
        include!("kiero.h");

        fn init(kind: RenderType) -> Status;
        fn shutdown();

        unsafe fn bind(index: u16, original: *mut *mut CVoid, function: *mut CVoid) -> Status;
        fn unbind(index: u16);

        #[cxx_name = "getRenderType"]
        fn get_render_type() -> RenderType;
        #[cxx_name = "getMethodsTable"]
        fn get_methods_table() -> *mut usize;
    }
}
