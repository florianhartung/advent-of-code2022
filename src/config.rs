use chrono::Datelike;

#[derive(clap::Parser, Debug)]
pub struct Args {
    day: Option<u32>,
    year: Option<u32>,

    remote: Option<RemoteArgs>,
}

#[derive(Debug)]
pub struct RemoteArgs {
    session: String,
    upload_answers: Option<bool>,
}

pub struct Config {
    pub day: u32,
    pub year: u32,
    pub remote_config: Option<RemoteConfig>,
}

pub struct RemoteConfig {
    pub session: String,
    pub upload_answers: bool,
}

impl From<RemoteArgs> for RemoteConfig {
    fn from(remote_args: RemoteArgs) -> Self {
        Self {
            session: remote_args.session,
            upload_answers: remote_args.upload_answers.unwrap_or(false),
        }
    }
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        let remote_config: Option<RemoteConfig> = args.remote.map(Into::into);

        let day = args.day
            .unwrap_or_else(|| chrono::Utc::now().day());
        let year = args.year
            .unwrap_or_else(|| chrono::Utc::now().year() as u32);

        Config {
            day,
            year,
            remote_config,
        }
    }
}
