use std::io::Write;
use log::{error, info, warn};
use log4rs;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::append::console::ConsoleAppender;
use log::LevelFilter;

/** A simple cli parser crate */
extern crate lapp;
pub struct Args {
    pub verbose: bool,
    pub polling: bool,
    #[allow(dead_code)]
    pub outfile : Box<dyn Write>,
    pub log4rs : log4rs::Handle,
}


pub fn parser() -> Args {
	let args = lapp::parse_args("
A game data parser for the game \"5D Chess With Multiverse Time Travel\"
It trys to find a a running windows process then reads ist memory. 
  -v, --verbose Output info logging to stdout, use a `log4rs.yaml` fore more control 
  -f, --outfile (outfile default stdout ) output file name
  -l, --logfile (string default '') log file name for defaulter logger overwritten by log4rs.yaml
  -p, --polling When set this program will poll the game every 1 second 
	");
	let verbose : bool = args.get_bool("verbose");
    let outfile : Box<dyn Write> = args.get_outfile("outfile");
    
    /*  Build a logger config */ 
    let stdout = ConsoleAppender::builder().build();
    let filename : String = args.get_string("logfile");
    let mut logfile : Box<dyn log4rs::append::Append> = Box::new(stdout);
    if !filename.is_empty() {     
        use log4rs::append::file::FileAppender;
        logfile = Box::new(FileAppender::builder().build(filename).unwrap());
    }
    // Building the default logger
    let default_logger_config = log4rs::Config::builder() 
        .appender(Appender::builder().build("logfile", logfile))
        .build(Root::builder().appender("logfile").build(LevelFilter::Warn))
        .unwrap();

    // Try and load a optional logging from disk
    let mut config  = match log4rs::config::load_config_file("log4rs.yaml", Default::default()) {
        Ok(r) => r,
        Err(_) => default_logger_config,
     };
     {
        let root = config.root_mut();
        root.set_level(LevelFilter::Error);
        if verbose {
            root.set_level(LevelFilter::Info);
        }
     }
    let log4rs = log4rs::init_config(config).unwrap();
    return Args {
        verbose,
        outfile,
        log4rs,
        polling :  args.get_bool("polling"),
    };
}