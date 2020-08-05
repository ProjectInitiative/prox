mod options;
mod config;
mod oshelper;


fn main() {
//     let matches = options::get_matches();

     // config::test_config();
     let pathbuf = oshelper::combine_paths(&["hell0","hi", "dev", "bruh","rust","is","hard"]);
     let test = pathbuf.into_os_string().into_string().unwrap();
     println!("{}",test);

    return;
//     // Gets a value for config if supplied by user, or defaults to "default.conf"
//     let config = matches.value_of("config").unwrap_or("default.conf");
//     println!("Value for config: {0}", config);

//     // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
//     // required we could have used an 'if let' to conditionally get the value)
//     println!("Using input file: {0}", matches.value_of("INPUT").unwrap());

//     // Vary the output based on how many times the user used the "verbose" flag
//     // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
//     match matches.occurrences_of("v") {
//         0 => println!("No verbose info"),
//         1 => println!("Some verbose info"),
//         2 => println!("Tons of verbose info"),
//         3 | _ => println!("Don't be crazy"),
//     }


    // more program logic goes here...
}