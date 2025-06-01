use clap::{Arg, ArgAction, ArgMatches, Command};
use color_print::cprintln;
use librs::fs::filemeta::m4a::Mp4a;

pub fn command() -> Command {
    Command::new("explain-format")
        .about("Explains the `format` arg")
        .arg(
            Arg::new("m4a")
                .help("Also list the parameters available for m4a files.")
                .action(ArgAction::SetTrue)
                .long("m4a")
        )
}

pub fn handle(matches: &ArgMatches) -> i32 {
    cprintln!(
        r#"The <yellow>`format`</> arg is the crux of this tool.

The file type is inferred from the file name, so including the extension in the format string is optional.

<bold,underline>Parameters</>

    - <yellow>%artist%</>
        The artist's name.
        <cyan>Default</> "Unknown Artist"

    - <yellow>%album%</>
        The Album Title.
        <cyan>Default</> "Unknown Album"

    - <yellow>%title%</>
        The Song Name.
        <cyan>Default</> "Unknown Song"

There are different parameters available for different file types, based on what metadata is available.

To get a list of the keys, pass the file extension as a parameter in the previous command (`--m4a`).
"#
    );

    let m4a = matches.get_one::<bool>("m4a").expect("has default");

    if *m4a {
        let hm = Mp4a::defaults_hashmap();

        cprintln!("<bold,underline>m4a parameters</>");

        let mut keys = hm
            .keys()
            .copied()
            .collect::<Vec<&str>>();
        keys.sort();

        for key in keys {
            println!("    {}", key);
        }
    }

    0
}
