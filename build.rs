#[cfg(windows)]
extern crate winres;

fn main() {
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("res/icon.ico");
        res.compile().unwrap();
    }
}
