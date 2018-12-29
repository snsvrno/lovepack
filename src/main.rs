extern crate clap;
#[macro_use] extern crate log;
extern crate pretty_env_logger;
#[macro_use] extern crate failure; use failure::Error;

extern crate lpsettings;
extern crate lprun;

extern crate updater_lp;

// upstream for the repository, used as the source of the releases
static UPDATER_URL : &str = "https://github.com/snsvrno/lpsettings-rs";

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
        .subcommand(lprun::interface::app().name("run"))
        .subcommand(lpsettings::interface::app().name("settings"))
        .subcommand(clap::SubCommand::with_name("update")
            .about("Updates the app"))

        // processes everything
        .get_matches();

    // starts the loggers & sets the filter level for the logs
    match pretty_env_logger::formatted_builder() {
        Err(error) => { println!("Failed to start logging: {}",error); },
        Ok(mut builder) => {
            let level = if app.is_present("debug") { 
                log::LevelFilter::Info 
            } else { 
                log::LevelFilter::Error 
            };

            builder
                .filter(None,level)
                .init();
        }
    }

    // checks if there are updates
    match check_for_updates() {
        Ok(status) => match status {
            Status::UpdateAvailable => println!("Update available, use 'lovepack update' to install."),
            _ => (),
        },
        Err(error) => error!("{}",error),
    }

    // processess the arguement matches.
    match app.subcommand() {
        ("run", Some(subm)) => { if let Err(error) = lprun::interface::process(&subm) { error!("{}",error); }},
        ("settings", Some(subm)) => { if let Err(error) = lpsettings::interface::process(&subm) { error!("{}",error); }},
        ("update", Some(_)) => { if let Err(error) = update_app() { error!("{}",error); }},
        _ => { }
    }
}

// UPDATING STUFF

pub enum Status {
    NoUpdate,
    DidNotCheckForUpdate,
    UpdateComplete,
    UpdateAvailable,
}

fn update_app() -> Result<Status,Error> {
    //! performs the actual update,
    //! 
    //! the user request to update the app.

    match update_get_version_link() {
        None => Ok(Status::NoUpdate),
        Some(link) => {
            match updater_lp::update_from_link(&link) {
                Err(error) => Err(format_err!("{}",error)),
                Ok(_) => Ok(Status::UpdateComplete),
            }
        }
    }
}

fn check_for_updates() -> Result<Status,Error> {
    //! automatic checking for update loop,
    //! 
    //! writes to the configuration to keep track of somethings
    
    // if an update is already available, then no need to do this part.
    if let Ok(Some(lpsettings::Type::Switch(true))) = lpsettings::get_value("lovepack.update.available") {
        return Ok(Status::UpdateAvailable);
    }

    match lpsettings::update::check_if_should_update("lovepack") {
        false => Ok(Status::DidNotCheckForUpdate),
        true => match update_get_version_link() {
            Some(_) => Ok(Status::UpdateAvailable),
            None => Ok(Status::NoUpdate),
        }
    } 
}

fn update_get_version_link() -> Option<String> {
    //! returns the link for the most recent version,
    //! 
    //! also does some setting of the settings file based on update frequency
    //! and if there is an update available or not.

    let pkg_ver = env!("CARGO_PKG_VERSION");
    match updater_lp::create_version(pkg_ver) {
        None => { warn!("Cannot create app version from {}, will not be checking for updates.",pkg_ver) },
        Some(app_version) => {
            info!("Checking for update, currently version {}",app_version);
            match updater_lp::get_link_for_latest(UPDATER_URL) {
                Err(error) => { error!("{}",error); },
                Ok((link,version)) => {
                    if version > app_version {
                        info!("update found: {}",version);

                        if let Err(error) = lpsettings::update::set_last_update_as_now("lovepack") {
                            error!("{}",error);
                        }

                        if let Err(error) = lpsettings::set_value("lovepack.update.available",&true) {
                            error!("{}",error);
                        }

                        return Some(link);
                    } else {
                        info!("no update found.");
                        
                        if let Err(error) = lpsettings::update::set_last_update_as_now("lovepack") {
                            error!("{}",error);
                        }

                        if let Err(error) = lpsettings::set_value("lovepack.update.available",&false) {
                            error!("{}",error);
                        }
                    }
                }
            }
        }
    }

    None
}