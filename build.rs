use std::env;

fn main() {
  println!("cargo:rustc-link-search=./lib");

  let target = env::var("TARGET").expect("TARGET environment variable not set.");

  match target.as_str() {
    "x86_64-unknown-linux-gnu" => println!("cargo:rustc-link-lib=raylib_x64"),
    "wasm32-unknown-unknown" => {
      println!("cargo:rustc-link-lib=raylib_wasm");
    }
    _ => println!("cargo:warning=Unsupported target architecture"),
  };
}
