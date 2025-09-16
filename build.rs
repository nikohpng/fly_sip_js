use std::fs;
use std::path::Path;
use std::io::Write;
fn main() {
  napi_build::setup();
  if cfg!(target_os = "windows") {
    let lib_path = Path::new("./lib/win/libfly_sip.lib");
    let parent_path = "./lib/win/";
    if !lib_path.exists() {
        println!("cargo:warning=fly_sip lib not found, downloading...");

        fs::create_dir_all(parent_path).unwrap();

        let url = "http://ali.hpng.site:18080/fly_sip/libfly_sip_win.zip";
        let response = ureq::get(url).call().unwrap();
        
        let full_path = format!("{}/libfly_sip_win.zip", parent_path);
        let mut file = fs::File::create(&full_path).unwrap();
        let mut body = response.into_body();
        file.write_all(& body.read_to_vec().unwrap()).unwrap();

        println!("cargo:warning=Downloaded {}", full_path);

        let zipfile = fs::File::open(&full_path).unwrap();
        let mut archive = zip::ZipArchive::new(zipfile).unwrap();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let outpath = Path::new(parent_path).join(file.name());

            if file.is_dir() {
                fs::create_dir_all(&outpath).unwrap();
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p).unwrap();
                    }
                }
                let mut outfile = fs::File::create(&outpath).unwrap();
                std::io::copy(&mut file, &mut outfile).unwrap();
            }
        }

        println!("cargo:warning=Unzipped {}", full_path);
    }
    println!("cargo:rustc-link-search=native=./lib/win/");
    println!("cargo:rustc-link-lib=dylib=libfly_sip");     
  } else if cfg!(target_os = "macos") {
      
  }
}
