
extern crate serde_json;
extern crate clap;

mod innout;
mod bodies;

/// Checks if value passed in to program argument is numeric. Returns a Result 
/// 
/// # Argument
/// * 'strng' - The value passed by the user
/// 
fn numeric_validator(strng: String) -> Result<(), String>{
    if strng.parse::<f64>().is_ok(){
        Ok(())
    } else {
        Err(String::from("Input is non-numeric"))
    }
}

fn main() {

    // Defines the input arguments from the cli
    let matches = clap::App::new("Parallel Orbital Simulation Environment (POSE)")
        .version("DEV0.1")
        .about("Simulation aimed to model the orbital environment around Earth for bodies at all magnitudes.")
        .args(&[
            clap::Arg::with_name("INPUT")
                .help("json file containing information on bodies at initilzation.")
                .required(true)
                .index(1),
            clap::Arg::with_name("out")
                .help("Main name of output files.")
                .short("o")
                .long("out")
                .value_name("FILE_NAME")
                .takes_value(true),
            clap::Arg::with_name("timei")
                .help("Initial time for simulation start must, be in iso time format.")
                .long("timei")
                .value_name("INITAL_TIME")
                .takes_value(true),
            clap::Arg::with_name("step")
                .help("Simulation time step interval in seconds")
                .short("s")
                .long("step")
                .value_name("STEP_INTERVAL")
                .takes_value(true)
                .default_value("0.1") // Simulation step time
                .validator(numeric_validator)
        ])
        .get_matches();

    let inpt_file = matches.value_of("INPUT").unwrap(); // Will always have INPUT
    let sim_bodies = innout::parse_inpt(inpt_file);
}
