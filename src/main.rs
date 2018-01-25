extern crate clap;

use clap::{App, ArgMatches, Arg};
use std::env;
use std::ffi::OsString;

// This is poorly named and is just enough here to hopefully 
// highlight the lifetime problem
struct MyApp {
    app: App<'static, 'static>
}

impl MyApp {
    pub fn new() -> MyApp {
        let a = App::new("fake")
                     .arg(Arg::with_name("test")
                        .takes_value(true)
                        .short("t")
                    )
                    .arg(Arg::with_name("foo")
                        .takes_value(true)
                        .short("f")
                    )
                    .version("v1.0-beta");

        MyApp { 
            app: a
        }
    }

    pub fn get_matches(&mut self) -> ArgMatches {
       let args: Vec<OsString> = env::args_os().collect();
       
       self.app.get_matches_from_safe_borrow(&mut args.iter())
               .unwrap_or_else(|e| {
                    e.exit();
               })
    }

    pub fn get_test(&mut self) -> Result<String, String> {
        if let Some(ref value) = self.get_matches().value_of("test") {
            Ok(value.to_string())
        } else {
            Err(String::from("Nope"))
        }
    }

    pub fn get_foo(&mut self) -> Result<String, String> {
        if let Some(ref value) = self.get_matches().value_of("foo") {
            Ok(value.to_string())
        } else {
            Err(String::from("Nope"))
        }
    }
}

fn main() {
    let mut cli = MyApp::new();

    println!("{:?} {:?}", cli.get_test(), cli.get_foo());
}
