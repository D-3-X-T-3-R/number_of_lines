extern crate clap;
extern crate linecount;
extern crate walkdir;

use linecount::count_lines;
use std::path::Path;
use walkdir::WalkDir;

mod config_params;
fn main() {
    const APP_NAME: &str = "number_of_lines";

    let mut total_lines: usize = 0;
    let mut total_computed_files: usize = 0;

    let config_params = config_params::get_configuration_parameters(APP_NAME);
    let file_ext_list: Vec<&str> = config_params.file_extension().split(",").collect();

    if !Path::new(config_params.dir_path()).exists() {
        panic!(
            "No directory found at path : {}. Invalid directory path!",
            config_params.dir_path()
        )
    }

    for file in WalkDir::new(config_params.dir_path())
        .into_iter()
        .filter_map(|file| file.ok())
    {
        if file.metadata().unwrap().is_file() {
            let file_name = file.file_name().to_string_lossy();
            if file_ext_list.is_empty() {
                get_line_count(file.path(), &mut total_lines);
                total_computed_files += 1;
            } else {
                for ext in file_ext_list.iter() {
                    if file_name.ends_with(ext.trim()) {
                        total_computed_files += 1;
                        get_line_count(file.path(), &mut total_lines);
                    }
                    break;
                }
            }
        }
    }

    println!(
        "Total {} number of lines present in {} number of files. ",
        total_lines, total_computed_files
    );
}

fn get_line_count(file_path: &Path, total_lines: &mut usize) {
    *total_lines += count_lines(std::fs::File::open(file_path).expect(&format!(
        "Could not read file at location : {}",
        file_path.display()
    )))
    .expect(&format!(
        "Could not count number of lines for file : {}",
        file_path.display()
    )) + 1;
}
