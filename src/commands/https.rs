//!
//!
//!
//!
//!
use clap::{ArgMatches, Args, Command, FromArgMatches};

use crate::{cli::Config, commands::https};
use common::errors::MegaResult;

use gateway::https::{http_server, HttpOptions};

pub fn cli() -> Command {
    HttpOptions::augment_args_for_update(Command::new("https").about("Start Git HTTPS server"))
}

#[tokio::main]
pub(crate) async fn exec(_config: Config, args: &ArgMatches) -> MegaResult {
    let server_matchers = HttpOptions::from_arg_matches(args)
        .map_err(|err| err.exit())
        .unwrap();
    println!("{server_matchers:#?}");
    https::http_server(&server_matchers).await.unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {}
