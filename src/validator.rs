use crate::*;

/// check if the provided path is valid output-file
pub fn is_output_ok(conf: &mut CliArgs, cwd: &Path) -> bool {
    if let Some(output_path_str) = &conf.output {
        // some output path has been entered
        let output_path = std::path::Path::new(&output_path_str);
        if output_path.exists() {
            // the specified output already exists => overwrite?
            if conf.force {
                // permission to overwrite has been granted
                println!(
                    "WARNING: The output file you entered already exists. It will be replaced."
                );
                if conf.verbose {
                    let Ok(output_abs_path) = output_path.canonicalize() else {
                        println!("ERROR: Transformation to absolute path failed.");
                        conf.output = None; // invalidate the input for next validation
                        return false;
                    };
                    println!("Accepted output file: \"{}\"", output_abs_path.display());
                }
                return true;
            } else {
                // ask for permission to overwrite the existing file
                println!("The output file you entered already exists and I don't have permission to overwrite it.");
                println!("Do you want me to overwrite it?");
                println!("Write \"y\" if YES, or anything else for NO.");
                if wait_for_input() == "y" {
                    conf.force = true; // user wants to overwrite, so enable --force
                    if conf.verbose {
                        let Ok(output_abs_path) = output_path.canonicalize() else {
                            println!("ERROR: Transformation to absolute path failed.");
                            conf.output = None; // invalidate the input for next validation
                            return false;
                        };
                        println!("Accepted output file: \"{}\"", output_abs_path.display());
                    }
                    // the file will be overwritten later when writing the actual data,
                    // so there is no need to do anything here
                    return true;
                } else {
                    conf.output = None; // user doesn't want to overwrite, so invalidate the output path and let them enter it again
                    return false;
                }
            }
        } else {
            // the specified output doesn't exist
            // try to create an empty file to test if the output directory exists
            match std::fs::write(output_path, "") {
                Ok(_) => {
                    // the creation was successful, so the output directory exists
                    // the next call of std::fs::write() will overwrite the empty file that was just created so it shouldn't be a problem
                    if conf.verbose {
                        let Ok(output_abs_path) = output_path.canonicalize() else {
                            println!("ERROR: Transformation to absolute path failed.");
                            conf.output = None; // invalidate the input for next validation
                            return false;
                        };
                        println!("Accepted output file: \"{}\"", output_abs_path.display());
                    }
                    return true;
                }
                Err(_) => {
                    // the creation of output file failed => most likely the target directory doesn't exist
                    println!("Sorry, I can't create the output file you specified.");
                    println!("Check if the directory exists and if you have write permissions.");
                    conf.output = None; // invalidate the input for next iteration
                    return false;
                }
            }
        }
    } else {
        // there is no output path specified
        println!(
            "Enter an output file (with absolute or relative path to \"{}\").",
            cwd.display()
        );
        conf.output = Some(wait_for_input());
        return false; // the user entered a new value, start new validation loop
    }
}

/// check if the provided path is valid input-file
pub fn is_input_ok(conf: &mut CliArgs, cwd: &Path) -> bool {
    if let Some(input_path) = &conf.input {
        let input_path = std::path::Path::new(&input_path);
        if !input_path.exists() {
            println!(
                "Sorry, the file \"{}\" does not exist.",
                input_path.display()
            );
            conf.input = None; // invalidate the input for next validation
            return false;
        } else {
            if conf.verbose {
                let Ok(input_abs_path) = input_path.canonicalize() else {
                    println!("ERROR: Transformation to absolute path failed.");
                    conf.input = None; // invalidate the input for next validation
                    return false;
                };
                println!("Accepted input file: \"{}\"", input_abs_path.display());
            }
            return true;
        }
    } else {
        println!(
            "Enter an input file (with absolute or relative path to \"{}\").",
            cwd.display()
        );
        conf.input = Some(wait_for_input());
        return false;
    }
}
