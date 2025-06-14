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
    println!("cargo:rerun-if-changed=raylib");

    let target = dbg!(std::env::var("TARGET")).unwrap();
    let out = PathBuf::from(dbg!(std::env::var("OUT_DIR")).unwrap());
    let rl_path = dbg!(out.join("raylib"));
    let bindings_path = dbg!(out.join("bindings.rs"));
    let profile = Profile::new();
    let platform = dbg!(Platform::new(&target));
    let platform_os = dbg!(PlatformOS::new(&target));

    // Copy Raylib to output
    copy_recursive("raylib", &rl_path)
        .unwrap_or_else(|e| panic!("failed to copy raylib source to `{}`: {e}", out.display()));

    // Build with cmake
    let mut conf = cmake::Config::new(&rl_path);

    conf.profile(profile.as_str())
        .define("CMAKE_BUILD_TYPE", profile.as_str())
        .define("BUILD_EXAMPLES", "OFF")
        .define("SUPPORT_BUSY_WAIT_LOOP", "OFF")
        .define("SUPPORT_FILEFORMAT_JPG", if cfg!(feature = "support_jpeg") { "ON" } else { "OFF" })
        .define("RAYMATH_STATIC_INLINE", "ON");

    if cfg!(feature = "custom_frame_control") {
        conf.define("SUPPORT_CUSTOM_FRAME_CONTROL", "ON");
    }

    // Enable wayland cmake flag if feature is specified
    if cfg!(feature = "wayland") {
        conf.define("USE_WAYLAND", "ON");
        conf.define("USE_EXTERNAL_GLFW", "ON");
    } else {
        conf.define("USE_WAYLAND", "OFF");
    }

    if cfg!(feature = "opengl_33") {
        conf.define("OPENGL_VERSION", "3.3");
    } else if cfg!(feature = "opengl_21") {
        conf.define("OPENGL_VERSION", "2.1");
    } else if /*cfg!(feature = "opengl_11")*/ false {
        conf.define("OPENGL_VERSION", "1.1");
    } else if cfg!(feature = "opengl_es_20") {
        conf.define("OPENGL_VERSION", "ES 2.0");
        println!("cargo:rustc-link-lib=GLESv2");
        println!("cargo:rustc-link-lib=GLdispatch");
    } else if cfg!(feature = "opengl_es_30") {
        conf.define("OPENGL_VERSION", "ES 3.0");
        println!("cargo:rustc-link-lib=GLESv2");
        println!("cargo:rustc-link-lib=GLdispatch");
    } else {
        conf.define("OPENGL_VERSION", "OFF");
    }

    conf.define("PLATFORM", match platform {
        Platform::Desktop if cfg!(feature = "sdl") => {
            println!("cargo:rustc-link-lib=SDL2");
            "SDL"
        }
        _ => platform.as_str(),
    });

    let dst = conf.build();
    // cmake lib directory
    let dst_lib = ["lib", "lib64", "lib32"]
        .into_iter()
        .find_map(|lib_dir| {
            let path = dst.join(lib_dir);
            path.exists().then_some(path)
        })
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

    // Generate bindings
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
        .blocklist_item("FP_(?:INFINITE|NAN|NORMAL|SUBNORMAL|ZERO)|IPPORT_RESERVED")
        .fit_macro_constants(true)
        .no_convert_floats()
        .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: false })
        .bitfield_enum(r".+Flags|Gesture")
        .constified_enum_module(r".+Index")
        .translate_enum_integer_types(true)
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

    println!("cargo:rustc-link-lib=static=raylib");

    // Link
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

    bindings
        .write_to_file(bindings_path)
        .expect("failed to write bindings");
}
