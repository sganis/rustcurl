// src/main.rs

mod curl;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let config = match curl::parse_args(&args) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error: {e}");
            curl::print_usage();
            std::process::exit(1);
        }
    };

    match curl::perform_request(&config) {
        Ok(response) => {
            if config.silent {
                if config.output.is_none() {
                    print!("{}", response.body_string());
                }
            } else if config.head_only {
                println!("Status: {}", response.status_code);
                println!();
                for header in &response.headers {
                    println!("{header}");
                }
                if let Some(ref timing) = response.timing {
                    println!();
                    print!("{timing}");
                }
            } else if config.output.is_some() {
                println!("Status: {}", response.status_code);
                println!();
                for header in &response.headers {
                    println!("{header}");
                }
                if let Some(ref path) = config.output {
                    println!();
                    println!("Body written to {path}");
                }
                if let Some(ref timing) = response.timing {
                    println!();
                    print!("{timing}");
                }
            } else {
                print!("{response}");
            }
        }
        Err(e) => {
            eprintln!("Request failed: {e}");
            std::process::exit(1);
        }
    }
}
