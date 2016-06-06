extern crate cmake;

fn add_squirrel_defines(cmake_cfg: &mut cmake::Config) {
	cmake_cfg.define("INSTALL_LIB_DIR", ".");
    if cfg!(feature = "use_double") {
        cmake_cfg.define("SQUSEDOUBLE", "");
    }
}

fn export_squirrel(path: std::path::PathBuf) {
    println!("cargo:rustc-link-search=native={}", path.display());
    println!("cargo:rustc-link-lib=static=squirrel_static");
}


fn main() {
    let mut cmake_cfg = cmake::Config::new("squirrel");
    add_squirrel_defines(&mut cmake_cfg);
    let path = cmake_cfg.build();
    export_squirrel(path);
}
