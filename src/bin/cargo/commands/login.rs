use crate::command_prelude::*;

use cargo::ops;

pub fn cli() -> App {
    subcommand("login")
        .about(
            "Save an api token from the registry locally. \
             If token is not specified, it will be read from stdin.",
        )
        .arg_quiet()
        .arg(Arg::with_name("token"))
        // --host is deprecated (use --registry instead)
        .arg(
            opt("host", "Host to set the token for")
                .value_name("HOST")
                .hidden(true),
        )
        .arg(opt("registry", "Registry to use").value_name("REGISTRY"))
        .after_help("Run `cargo help login` for more detailed information.\n")
}

pub fn exec(config: &mut Config, args: &ArgMatches<'_>) -> CliResult {
    ops::registry_login(
        config,
        args.value_of("token").map(String::from),
        args.value_of("registry").map(String::from),
    )?;
    Ok(())
}
