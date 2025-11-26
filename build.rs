use std::env;

#[inline]
fn windows_gnu_lib(lib_name: &str) {
  println!("cargo:rustc-link-lib={}", lib_name);
  println!("cargo:rustc-link-lib=shell32");
  println!("cargo:rustc-link-lib=gdi32");
  println!("cargo:rustc-link-lib=winmm");
}

#[inline]
fn windows_msvc_lib(lib_name: &str) {
  println!("cargo:rustc-link-lib={}_msvc", lib_name);
  println!("cargo:rustc-link-lib=shell32");
  println!("cargo:rustc-link-lib=user32");
  println!("cargo:rustc-link-lib=gdi32");
  println!("cargo:rustc-link-lib=winmm");
}

fn main() {
  println!("cargo:rustc-link-search=./lib");

  let target = env::var("TARGET").expect("TARGET environment variable not set.");
  let mut platform = "DESKTOP";

  match target.as_str() {
    "i686-pc-windows-gnu" => windows_gnu_lib("raylib_w32"),
    "i686-pc-windows-msvc" => windows_msvc_lib("raylib_w32"),
    "i686-unknown-linux-gnu" => println!("cargo:rustc-link-lib=raylib_x86"),
    "x86_64-pc-windows-gnu" => windows_gnu_lib("raylib_w64"),
    "x86_64-pc-windows-msvc" => windows_msvc_lib("raylib_w64"),
    "x86_64-unknown-linux-gnu" => println!("cargo:rustc-link-lib=raylib_x64"),
    "wasm32-unknown-unknown" => {
      println!("cargo:rustc-link-lib=raylib_wasm");
      platform = "WEB";
    }
    _ => println!("cargo:warning=Unsupported target architecture"),
  };

  let profile = env::var("PROFILE");

  if let Ok(profile) = profile {
    println!("cargo:rustc-env=PROFILE={}", profile)
  }

  println!("cargo:rustc-env=PLATFORM={}", platform);
}
