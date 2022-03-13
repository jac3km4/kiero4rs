use std::path::{Path, PathBuf};

type Result<A> = std::result::Result<A, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    println!("cargo:rustc-link-lib=User32");
    #[cfg(feature = "d3d9")]
    println!("cargo:rustc-link-lib=d3d9");
    #[cfg(feature = "d3d10")]
    println!("cargo:rustc-link-lib=d3d10");
    #[cfg(feature = "d3d11")]
    println!("cargo:rustc-link-lib=d3d11");
    #[cfg(feature = "d3d12")]
    println!("cargo:rustc-link-lib=d3d12");
    #[cfg(feature = "opengl")]
    println!("cargo:rustc-link-lib=opengl32");
    #[cfg(feature = "vulkan")]
    println!("cargo:rustc-link-lib=vulkan");

    let mut bridge = cxx_build::bridge("src/ffi.rs");
    #[cfg(feature = "d3d9")]
    bridge.define("KIERO_INCLUDE_D3D9", "1");
    #[cfg(feature = "d3d10")]
    bridge.define("KIERO_INCLUDE_D3D10", "1");
    #[cfg(feature = "d3d11")]
    bridge.define("KIERO_INCLUDE_D3D11", "1");
    #[cfg(feature = "d3d12")]
    bridge.define("KIERO_INCLUDE_D3D12", "1");
    #[cfg(feature = "opengl")]
    bridge.define("KIERO_INCLUDE_OPENGL", "1");
    #[cfg(feature = "vulkan")]
    bridge.define("KIERO_INCLUDE_VULKAN", "1");
    #[cfg(feature = "minhook")]
    bridge.define("KIERO_USE_MINHOOK", "1");

    let kiero_files = generate_kiero_files()?;
    bridge
        .include("deps/kiero")
        .include("deps/glue")
        .include(&kiero_files)
        .file(kiero_files.join("kiero.cpp"));

    #[cfg(feature = "minhook")]
    {
        let minhook_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("deps/kiero/minhook/src");
        let files = vec![
            minhook_dir.join("buffer.c"),
            minhook_dir.join("hook.c"),
            minhook_dir.join("trampoline.c"),
            minhook_dir.join("hde/hde32.c"),
            minhook_dir.join("hde/hde64.c"),
        ];
        bridge.files(files);
    }

    bridge.compile("kiero");
    println!("cargo:rerun-if-changed=src/ffi.rs");
    println!("cargo:rerun-if-changed=deps/glue/glue.h");
    Ok(())
}

fn generate_kiero_files() -> Result<PathBuf> {
    let kiero_src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("deps/kiero");
    let kiero_tmp = tempdir::TempDir::new("kiero")?.into_path();
    let kiero_header = generate_kiero_header(&kiero_src.join("kiero.h"))?;

    std::fs::write(&kiero_tmp.join("kiero.h"), kiero_header)?;
    std::fs::copy(kiero_src.join("kiero.cpp"), kiero_tmp.join("kiero.cpp"))?;
    Ok(kiero_tmp)
}

fn generate_kiero_header(path: &Path) -> Result<String> {
    let contents = std::fs::read_to_string(path)?;

    // get rid of the default disabled flags, so that we can override them with compiler flags
    let res = contents
        .replace("#define KIERO_INCLUDE_D3D9   0", "")
        .replace("#define KIERO_INCLUDE_D3D10  0", "")
        .replace("#define KIERO_INCLUDE_D3D11  0", "")
        .replace("#define KIERO_INCLUDE_D3D12  0", "")
        .replace("#define KIERO_INCLUDE_OPENGL 0", "")
        .replace("#define KIERO_INCLUDE_VULKAN 0", "")
        .replace("#define KIERO_USE_MINHOOK    0", "");

    Ok(res)
}
