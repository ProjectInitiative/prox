mod options;
mod config;
mod oshelper;


fn main() {
    let matches = options::get_matches();

    config::read_config();
    // let pathbuf = oshelper::combine_paths(&["hell0","hi", "dev", "bruh","rust","is","hard"]);
    // let test = pathbuf.into_os_string().into_string().unwrap();
    // println!("{}",test);

    return;
}