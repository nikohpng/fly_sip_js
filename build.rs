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
    let lib_path = Path::new("./lib/macos/FlySip.xcframework");
    let parent_path = "./lib/macos/";
    if !lib_path.exists() {
        println!("cargo:warning=FlySip.xcframework not found, downloading...");

        fs::create_dir_all(parent_path).unwrap();

        let url = "http://ali.hpng.site:18080/fly_sip/FlySip.xcframework.zip";
        let full_path = format!("{}/FlySip.xcframework.zip", parent_path);
       
       let status = std::process::Command::new("curl")
                .arg("-L")            // 跟随重定向
                .arg("-o")
                .arg(&full_path)      // 输出文件路径
                .arg(url)             // 下载 URL
                .status()
                .expect("Failed to execute curl");

        if !status.success() {
            panic!("curl failed with status: {}", status);
        }

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
    // 项目根目录绝对路径
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let framework_path = format!("{}/lib/macos", manifest_dir);
    //println!("cargo:warning=Project root: {}", framework_path);
    println!("cargo:rustc-link-search=framework={}/FlySip.xcframework/macos-arm64_x86_64", framework_path);
    println!("cargo:rustc-link-lib=framework=FlySip");   
  }
}
