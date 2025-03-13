use std::error::Error;
use std::process::ExitCode;

fn main() -> ExitCode {
    // Attempt to gracfully exit the run process, and store the code for future use
    let exit_code = match run() {
        Ok(run_code) => run_code,
        Err(run_err) => {
            // TODO: I future check if the error was a broken pipeline
            // NOTE: For now this logs the error to the console and exits gracefully with a general error exit code
            eprintln!("Error: {run_err}");
            return ExitCode::from(1);
        }
    };

    // Return the computed ExitCode to terminate the process (ExitCode implements the Termination trait)
    return exit_code;
}

fn run() -> Result<ExitCode, Box<dyn Error>> {
    // Temporarily exit with success
    Ok(ExitCode::from(0))
}
