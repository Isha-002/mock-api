use clap::{command, Arg, ArgAction, ArgMatches};

pub fn arguments() -> ArgMatches {
    command!()
        .about("Restaurant Api")
        // you can pass arguments like this:
        .arg(
            // -d or --db-url
            Arg::new("database url")
                .short('d')
                .long("db-url")
                .aliases(["db", "url", "database", "psql", "dburl", "db_url"])
                .help("a url that connects your postgres database to the server"),
        )
        .arg(
            // --reset
            Arg::new("reset")
                .long("reset")
                .help("delete database tables and applying new changes")
                .action(ArgAction::SetTrue),
        )
        .arg(
            // --fake data
            Arg::new("data")
                .long("data")
                .help("inserts fake data into database")
                .action(ArgAction::SetTrue),
        )
        .get_matches()
}
