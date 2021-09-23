use clap::{App, AppSettings, Arg, SubCommand};
use harvest::{tasks, timer, Client, Config, Result};

const TIMER_SUBCOMMAND: &str = "timer";
const TIMER_CURRENT_SUBCOMMAND: &str = "current";
const TIMER_CURRENT_CONTINUOUS_ARG: &str = "continuous";
const TIMER_TOGGLE_SUBCOMMAND: &str = "toggle";

const TASK_SUBCOMMAND: &str = "task";
const TASK_LIST_SUBCOMMAND: &str = "list";

#[tokio::main]
async fn main() -> Result<()> {
    let matches = app().get_matches();

    let config = Config::from_file()?;
    let client = Client::new(&config.auth)?;

    if let Some(matches) = matches.subcommand_matches(TIMER_SUBCOMMAND) {
        match matches.subcommand_name() {
            Some(TIMER_CURRENT_SUBCOMMAND) => {
                let continuous = matches
                    .subcommand_matches(TIMER_CURRENT_SUBCOMMAND)
                    .map_or(false, |matches| {
                        matches.is_present(TIMER_CURRENT_CONTINUOUS_ARG)
                    });
                if continuous {
                    timer::show_current_continuous(&client).await;
                } else {
                    timer::show_current(&client).await?;
                }
            }
            Some(TIMER_TOGGLE_SUBCOMMAND) => timer::show_toggle(&client).await?,
            _ => unreachable!(),
        };
    } else if let Some(matches) = matches.subcommand_matches(TASK_SUBCOMMAND) {
        match matches.subcommand_name() {
            Some(TASK_LIST_SUBCOMMAND) => {
                tasks::list(&client).await?;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}

fn app() -> App<'static, 'static> {
    App::new("Harvest")
        .version("0.1")
        .author("Peter Stuart <peter@peterstuart.org>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name(TIMER_SUBCOMMAND)
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name(TIMER_CURRENT_SUBCOMMAND)
                        .about("Get the current timer")
                        .arg(
                            Arg::with_name(TIMER_CURRENT_CONTINUOUS_ARG)
                                .short("c")
                                .long("continuous")
                                .takes_value(false),
                        ),
                )
                .subcommand(
                    SubCommand::with_name(TIMER_TOGGLE_SUBCOMMAND)
                        .about("Toggle the current timer on or off"),
                ),
        )
        .subcommand(
            SubCommand::with_name(TASK_SUBCOMMAND)
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name(TASK_LIST_SUBCOMMAND).about("List all assigned tasks"),
                ),
        )
}
