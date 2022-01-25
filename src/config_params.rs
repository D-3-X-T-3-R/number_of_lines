use clap::{App, Arg};

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    dir_path: String,
    file_extensions: String,
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let dir_path = matches
            .value_of("dir_path")
            .expect("Error getting Directory Path.")
            .to_string();

        let file_extensions = matches
            .value_of("file_extensions")
            .expect("Error getting File Extension.")
            .to_string();

        ConfigurationParameters {
            dir_path,
            file_extensions,
        }
    }
}

impl ConfigurationParameters {
    pub fn dir_path(&self) -> &str {
        &self.dir_path
    }
    pub fn file_extension(&self) -> &str {
        &self.file_extensions
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("Get custom arguments")
        .arg(
            Arg::new("dir_path")
                .long("dir-path")
                .value_name("Directory Path")
                .help("Path to the directory.")
                .required(true),
        )
        .arg(
            Arg::new("file_extensions")
                .long("file-extensions")
                .value_name("File extensions list")
                .help("Value of file extensions")
                .default_value("")
                .required(false),
        )
        .get_matches()
}
