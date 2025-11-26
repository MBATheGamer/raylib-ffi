use std::env;

fn main() {
  println!("cargo:rustc-link-search=./lib");

  let target = env::var("TARGET").expect("TARGET environment variable not set.");
  let mut platform = "DESKTOP";

  match target.as_str() {
    "x86_64-unknown-linux-gnu" => println!("cargo:rustc-link-lib=raylib_x64"),
    "wasm32-unknown-unknown" => {
      println!("cargo:rustc-link-lib=raylib_wasm");
      platform = "WEB";
    }
    _ => println!("cargo:warning=Unsupported target architecture"),
  };

  println!("cargo:rustc-env=PLATFORM={}", platform);
}
