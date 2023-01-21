#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket; 

use std::io;
use std::path::*; 
use rocket::response::NamedFile;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    let path = format!("{}/../frontend/dist", env!("CARGO_MANIFEST_DIR"));

    NamedFile::open(Path::new(&path).join("index.html"))
}

#[get("/<file..>")]
fn files(file: PathBuf) -> io::Result<NamedFile> {
    let path = format!("{}/../frontend/dist", env!("CARGO_MANIFEST_DIR"));

    NamedFile::open(Path::new(&path).join(file))
}


fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
