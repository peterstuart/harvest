use clap::{value_t, AppSettings, Arg, SubCommand};
use harvest::{tasks, timer, Client, Config, Id, Project, Result, Task};

type App = clap::App<'static, 'static>;

const TIMER_SUBCOMMAND: &str = "timer";

const TIMER_CURRENT_SUBCOMMAND: &str = "current";
const TIMER_CURRENT_CONTINUOUS_ARG: &str = "continuous";

const TIMER_TOGGLE_SUBCOMMAND: &str = "toggle";

const TIMER_START_SUBCOMMAND: &str = "start";
const TIMER_START_PROJECT: &str = "project";
const TIMER_START_TASK: &str = "task";

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
                    .unwrap()
                    .is_present(TIMER_CURRENT_CONTINUOUS_ARG);
                if continuous {
                    timer::show_current_continuous(&client).await;
                } else {
                    timer::show_current(&client).await?;
                }
            }
            Some(TIMER_TOGGLE_SUBCOMMAND) => timer::show_toggle(&client).await?,
            Some(TIMER_START_SUBCOMMAND) => {
                let matches = matches.subcommand_matches(TIMER_START_SUBCOMMAND).unwrap();
                let project_id: Id<Project> =
                    value_t!(matches, TIMER_START_PROJECT, Id<Project>).unwrap();
                let task_id = value_t!(matches, TIMER_START_TASK, Id<Task>).unwrap();

                timer::show_start(&client, project_id, task_id).await?;
            }
            _ => unreachable!(),
        };
    } else if let Some(matches) = matches.subcommand_matches(TASK_SUBCOMMAND) {
        match matches.subcommand_name() {
            Some(TASK_LIST_SUBCOMMAND) => {
                tasks::list(&client).await?;
            }
            _ => unreachable!(),
        };
    } else {
        unreachable!();
    }

    Ok(())
}

fn app() -> App {
    App::new("Harvest")
        .version("0.1")
        .author("Peter Stuart <peter@peterstuart.org>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(timer_subcommand())
        .subcommand(task_subcommand())
}

fn timer_subcommand() -> App {
    SubCommand::with_name(TIMER_SUBCOMMAND)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(timer_current_subcommand())
        .subcommand(timer_toggle_subcommand())
        .subcommand(timer_start_subcommand())
}

fn timer_current_subcommand() -> App {
    SubCommand::with_name(TIMER_CURRENT_SUBCOMMAND)
        .about("Gets the current timer")
        .arg(
            Arg::with_name(TIMER_CURRENT_CONTINUOUS_ARG)
                .help("Prints the current timer every few seconds")
                .short("c")
                .long("continuous")
                .takes_value(false),
        )
}

fn timer_toggle_subcommand() -> App {
    SubCommand::with_name(TIMER_TOGGLE_SUBCOMMAND).about("Toggles the current timer on or off")
}

fn timer_start_subcommand() -> App {
    SubCommand::with_name(TIMER_START_SUBCOMMAND)
        .about("Starts or resumes a timer")
        .arg(
            Arg::with_name(TIMER_START_PROJECT)
                .help("Project ID")
                .short("p")
                .long("project")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name(TIMER_START_TASK)
                .help("Task ID")
                .short("t")
                .long("task")
                .required(true)
                .takes_value(true),
        )
}

fn task_subcommand() -> App {
    SubCommand::with_name(TASK_SUBCOMMAND)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(task_list_subcommand())
}

fn task_list_subcommand() -> App {
    SubCommand::with_name(TASK_LIST_SUBCOMMAND).about("List all assigned tasks")
}
