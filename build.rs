fn main() {
  napi_build::setup();
  println!("cargo:rustc-link-search=native=./lib");  // 存放 .lib/.dll.a 的目录
  println!("cargo:rustc-link-lib=dylib=libfly_sip");     
}
