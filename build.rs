fn main() {
    // pkg-config 会自动查找标准路径
    // 只需要确保链接搜索路径正确
    
    #[cfg(target_os = "macos")]
    {
        use std::path::PathBuf;
        
        // Apple Silicon
        if PathBuf::from("/opt/homebrew/lib").exists() {
            println!("cargo:rustc-link-search=/opt/homebrew/lib");
        } 
        // Intel Mac
        else if PathBuf::from("/usr/local/lib").exists() {
            println!("cargo:rustc-link-search=/usr/local/lib");
        }
    }
    
    println!("cargo:rerun-if-changed=build.rs");
}