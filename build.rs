use std::{fs, path::{Path, PathBuf}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Profile {
    Debug,
    Release,
}

impl Profile {
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

fn uname() -> String {
    let v = std::process::Command::new("uname")
        .output()
        .expect("failed to run uname")
        .stdout;

    String::from_utf8(v)
        .expect("uname expected in utf-8")
}

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

fn main() {
    println!("cargo:rerun-if-changed=./binding/binding.h");

    let target = std::env::var("TARGET").unwrap();
    let out = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    let profile = if cfg!(debug_assertions) {
        Profile::Debug
    } else {
        Profile::Release
    };

    let platform = if target.contains("wasm") {
        Platform::Web
    } else if target.contains("armv7-unknown-linux") {
        Platform::RaspberryPi
    } else {
        Platform::Desktop
    };

    let platform_os = match platform {
        Platform::Desktop => {
            if target.contains("windows") || std::env::var("OS").is_ok_and(|os| os.contains("Windows_NT")) {
                PlatformOS::Windows
            } else {
                let uname = uname();
                match uname.as_str() {
                    "Linux" => PlatformOS::Linux,
                    "Darwin" => PlatformOS::OSX,
                    | "FreeBSD"
                    | "OpenBSD"
                    | "NetBSD"
                    | "DragonFly"
                        => PlatformOS::BSD,
                    _ => PlatformOS::Unknown,
                }
            }
        }

        Platform::RaspberryPi => {
            let uname = uname();
            match uname.as_str() {
                "Linux" => PlatformOS::Linux,
                _ => PlatformOS::Unknown,
            }
        }

        Platform::Web => PlatformOS::Unknown,
    };

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
        .header("binding/binding.h")
        .clang_arg("-std=c99")
        .clang_arg(platform.clang_arg());

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
        .expect("Unable to generate bindings");

    println!("cargo:rustc-link-lib=static=raylib");
    bindings
        .write_to_file(out.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
