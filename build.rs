use std::{fs, path::{Path, PathBuf}};

fn copy_recursive<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> std::io::Result<()> {
    if !to.as_ref().exists() {
        std::fs::create_dir_all(&to)?;
        for entry in fs::read_dir(from)? {
            let entry = entry?;
            let ty = entry.file_type()?;
            let from = entry.path();
            let to = to.as_ref().join(entry.file_name());
            if ty.is_dir() {
                copy_recursive(from, to)?;
            } else {
                std::fs::copy(from, to)?;
            }
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Profile {
    Debug,
    Release,
}

impl Profile {
    const fn new() -> Self {
        if cfg!(debug_assertions) {
            Self::Debug
        } else {
            Self::Release
        }
    }

    #[inline]
    const fn as_str(&self) -> &'static str {
        match self {
            Self::Debug => "Debug",
            Self::Release => "Release",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Platform {
    Desktop,
    Web,
    RaspberryPi,
}

impl Platform {
    fn new(target: &str) -> Self {
        if target.contains("wasm") {
            Self::Web
        } else if target.contains("armv7-unknown-linux") {
            Self::RaspberryPi
        } else {
            Self::Desktop
        }
    }

    #[inline]
    const fn as_str(&self) -> &'static str {
        match self {
            Self::Desktop => "Desktop",
            Self::Web => "Web",
            Self::RaspberryPi => "Raspberry Pi",
        }
    }

    #[inline]
    const fn clang_arg(&self) -> &'static str {
        match self {
            Self::Desktop => "-DPLATFORM_DESKTOP",
            Self::Web => "-DPLATFORM_WEB",
            Self::RaspberryPi => "-DPLATFORM_RPI",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PlatformOS {
    Windows,
    Linux,
    BSD,
    OSX,
    Unknown,
}

impl PlatformOS {
    const LOOKUP: [(&[&str], Self); 4] = [
        (&["windows"], Self::Windows),
        (&["linux"], Self::Linux),
        (&["ios", "macos"], Self::OSX),
        (&["freebsd", "openbsd", "netbsd", "dragonfly"], Self::BSD),
    ];

    fn new(target: &str) -> Self {
        Self::LOOKUP
            .into_iter()
            .find_map(|(list, os)|
                list.into_iter()
                    .any(|s| target.contains(s))
                    .then_some(os)
            )
            .unwrap_or(Self::Unknown)
    }

    #[allow(dead_code)]
    const fn as_str(&self) -> &'static str {
        match self {
            Self::Windows => "Windows",
            Self::Linux => "Linux",
            Self::BSD => "BSD",
            Self::OSX => "OSX",
            Self::Unknown => "Unknown",
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=./binding/binding.h");
    println!("cargo:rerun-if-changed=raylib");

    let target = dbg!(std::env::var("TARGET")).unwrap();
    let out = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    let profile = Profile::new();
    let platform = Platform::new(&target);
    let platform_os = PlatformOS::new(&target);

    let rl_path = out.join("raylib");
    if !rl_path.exists() {
        copy_recursive("raylib", rl_path.as_path())
            .unwrap_or_else(|e| panic!("failed to copy raylib source to `{}`: {e}", out.display()));
    }

    let mut conf = cmake::Config::new(rl_path.as_path());
    conf.profile(profile.as_str())
        .define("PLATFORM", platform.as_str())
        .define("CMAKE_BUILD_TYPE", profile.as_str())
        .define("BUILD_EXAMPLES", "OFF")
        .define("SUPPORT_BUSY_WAIT_LOOP", "OFF")
        .define("SUPPORT_FILEFORMAT_JPG", if cfg!(feature = "support_jpeg") { "ON" } else { "OFF" })
        .define("RAYMATH_STATIC_INLINE", "ON");

    if cfg!(feature = "custom_frame_control") {
        conf.define("SUPPORT_CUSTOM_FRAME_CONTROL", "ON");
    }

    let dst = conf.build();
    let dst_lib = ["lib", "lib64", "lib32"]
        .into_iter()
        .map(|lib_dir| dst.join(lib_dir))
        .find(|path| path.exists())
        .unwrap_or(dst);

    if matches!(platform_os, PlatformOS::Windows) {
        if let (from, Some(to)) = [
            ("raylib.lib", None),
            ("raylib_static.lib", Some("raylib.lib")),
            ("libraylib_static.a", Some("libraylib.a")),
            ("libraylib.a", None),
        ]
            .into_iter()
            .find_map(|(from, to)| {
                let from = dst_lib.join(from);
                from.exists().then_some((from, to))
            })
            .expect("failed to locate windows library")
        {
            std::fs::copy(from, dst_lib.join(to))
                .expect("failed to create windows library");
        }
    }

    if matches!(platform, Platform::Web) {
        let lib = dst_lib.join("libraylib.a");
        if !lib.exists() {
            std::fs::copy(dst_lib.join("libraylib.bc"), lib)
                .expect("failed to create wasm library");
        }
    }

    println!("cargo:rustc-link-search=native={}", dst_lib.display());

    let mut builder = bindgen::Builder::default()
        .headers([
            "raylib/src/config.h",
            "raylib/src/raylib.h",
            "raylib/src/raymath.h",
            "raylib/src/rcamera.h",
            "raylib/src/rgestures.h",
            "raylib/src/rlgl.h",
            "raylib/src/utils.h",
        ])
        .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
        .fit_macro_constants(true)
        .no_convert_floats()
        .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: false })
        .bitfield_enum(r".+Flags")
        .constified_enum_module(r".+Index")
        .prepend_enum_name(false)
        .array_pointers_in_arguments(true)
        .derive_default(true)
        .derive_copy(true)
        .derive_eq(true)
        .derive_hash(true)
        .generate_comments(true)
        .enable_function_attribute_detection()
        .generate_block(true)
        .generate_cstr(true)
        .merge_extern_blocks(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_arg("-std=c99")
        .clang_arg(platform.clang_arg())
        .clang_arg("-I../raylib/src");

    builder = match (platform, platform_os) {
        (Platform::Desktop, PlatformOS::Windows) => {
            builder
                .clang_arg("-D__STDC__")
        }
        (Platform::Web, _) => {
            builder
                .clang_arg("-fvisibility=default")
                .clang_arg("--target=wasm32-emscripten")
        }
        _ => builder,
    };

    let bindings = builder
        .generate()
        .expect("failed to generate bindings");

    match platform_os {
        PlatformOS::Windows => {
            println!("cargo:rustc-link-lib=dylib=winmm");
            println!("cargo:rustc-link-lib=dylib=gdi32");
            println!("cargo:rustc-link-lib=dylib=user32");
            println!("cargo:rustc-link-lib=dylib=shell32");
        }
        PlatformOS::Linux => {
            if cfg!(feature = "wayland") {
                println!("cargo:rustc-link-search=/usr/local/lib");
                println!("cargo:rustc-link-lib=wayland-client");
                println!("cargo:rustc-link-lib=glfw");
            } else if cfg!(target_os = "android") {
                println!("cargo:rustc-link-search=/usr/local/lib");
                println!("cargo:rustc-link-lib=X11");
            }
        }
        PlatformOS::OSX => {
            println!("cargo:rustc-link-search=native=/usr/local/lib");
            println!("cargo:rustc-link-lib=framework=OpenGL");
            println!("cargo:rustc-link-lib=framework=Cocoa");
            println!("cargo:rustc-link-lib=framework=IOKit");
            println!("cargo:rustc-link-lib=framework=CoreFoundation");
            println!("cargo:rustc-link-lib=framework=CoreVideo");
        }
        _ => {}
    }

    match platform {
        Platform::Web => {
            println!("cargo:rustc-link-lib=glfw");
        }
        Platform::RaspberryPi => {
            println!("cargo:rustc-link-search=/opt/vc/lib");
            println!("cargo:rustc-link-lib=bcm_host");
            println!("cargo:rustc-link-lib=brcmEGL");
            println!("cargo:rustc-link-lib=brcmGLESv2");
            println!("cargo:rustc-link-lib=vcos");
        }
        _ => {}
    }

    println!("cargo:rustc-link-lib=static=raylib");
    bindings
        .write_to_file(out.join("bindings.rs"))
        .expect("failed to write bindings");
}
