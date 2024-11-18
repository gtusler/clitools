use super::init;
use clap::ArgMatches;
use clap_complete::Shell;
use librs::cli::gen_completion::print_completions;

pub fn handle(matches: &ArgMatches) -> i32 {
    if let Some(generator) = matches.get_one::<Shell>("generator").copied() {
        let mut cmd = init::command();
        print_completions(generator, &mut cmd);
        return 0;
    }

    1
}
