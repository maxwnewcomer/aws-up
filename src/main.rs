use clap::{Arg, Command};
use serde::Serialize;
use std::fs;

#[derive(Serialize)]
struct TfVars {
    ami_id: String,
    instance_type: String,
    volume_size: u16,
    aws_region: String,
}

fn main() {
    let matches = Command::new("TF Configurator")
        .version("0.1")
        .author("Your Name <you@example.com>")
        .about("Configures Terraform variables for AWS environment")
        .arg(
            Arg::new("ami_id")
                .long("ami-id")
                .short('a')
                .default_value("ami-0c55b159cbfafe1f0")
                .help("AMI ID to use for the instance"),
        )
        .arg(
            Arg::new("instance_type")
                .long("instance-type")
                .short('i')
                .default_value("t2.micro")
                .help("Instance type to launch"),
        )
        .arg(
            Arg::new("volume_size")
                .long("volume-size")
                .short('v')
                .default_value("8")
                .help("Volume size in GB for the root disk"),
        )
        .arg(
            Arg::new("aws_region")
                .long("aws-region")
                .short('r')
                .default_value("us-east-1")
                .help("AWS region to deploy in"),
        )
        .get_matches();

    let ami_id = matches.get_one::<String>("ami_id").unwrap().to_string();
    let instance_type = matches
        .get_one::<String>("instance_type")
        .unwrap()
        .to_string();
    let volume_size_str = matches.get_one::<String>("volume_size").unwrap();
    let volume_size: u16 = volume_size_str
        .parse()
        .expect("volume_size must be a number");
    let aws_region = matches.get_one::<String>("aws_region").unwrap().to_string();

    let vars = TfVars {
        ami_id,
        instance_type,
        volume_size,
        aws_region,
    };

    let json = serde_json::to_string_pretty(&vars).expect("Failed to serialize tfvars");

    fs::write("terraform.tfvars.json", json).expect("Failed to write terraform.tfvars.json");

    println!("Updated terraform.tfvars.json successfully!");
}
