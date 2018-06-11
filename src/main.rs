extern crate clap;
extern crate ansi_term;

extern crate lpsettings;
extern crate lprun;
extern crate lmake;

use std::env;

fn main() {  
  // builds the app
  let app = clap::App::new("lovepack")
    .version(env!("CARGO_PKG_VERSION"))
    .author("snsvrno<snsvrno@tuta.io")
    .about("Toolset for working with love2D projects.")
    .name("lovepack")

  // arguements
    .arg(clap::Arg::with_name("debug").long("debug").help("Shows additional information about commands run."))

  // adding the subapps
    .subcommand(lmake::interface::app().name("library"))
    .subcommand(lpsettings::interface::app().name("settings"))
    .subcommand(lprun::interface::app().name("run"))

  // processes everything
    .get_matches();

  // checks if debug flag should be enabled
  if app.is_present("debug") { env::set_var("OUTPUT_DEBUG_ENABLED","true"); }

  // processess the arguement matches.
  match app.subcommand() {
    ("run", Some(subm)) => { if let Err(error) = lprun::interface::process(&subm) { println!("{}",error); }},
    ("settings", Some(subm)) => { if let Err(error) = lpsettings::interface::process(&subm) { println!("{}",error); }},
    ("library", Some(subm)) => { if let Err(error) = lmake::interface::process(&subm) { println!("{}",error); }},
    _ => { }
  }
}