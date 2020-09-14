extern crate dirs;

mod options;
mod config;
mod oshelper;

fn main() {
    let matches = options::get_matches();

    config::read_config();
    // let pathbuf = oshelper::combine_paths(&["hell0","hi", "dev", "bruh","rust","is","hard"]);
    // let test = pathbuf.into_os_string().into_string().unwrap();
    // println!("{}",test);

    match dirs::config_dir() {
        Some(path) => 
        {
            println!("Your home directory, probably: {}", path.display());
            let test = oshelper::combine_paths(&[&path.to_str().unwrap(), "prox"]);
            println!("{:?}", test.to_str().unwrap());
        }
        None => println!("Impossible to get your home dir!"),
    }

    return;
}