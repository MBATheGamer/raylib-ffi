use std::env;

fn main() {
  println!("cargo:rustc-link-search=./lib");

  let target = env::var("TARGET").expect("TARGET environment variable not set.");
  let mut platform = "DESKTOP";

  match target.as_str() {
    "i686-unknown-linux-gnu" => println!("cargo:rustc-link-lib=raylib_x86"),
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
