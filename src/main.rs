use std::process::{Command, exit};
use std::fs;

fn check_and_install_sudo() {
    // Check if sudo is installed
    let check_sudo = Command::new("sudo")
        .arg("--version")
        .output();

    match check_sudo {
        Ok(_) => {
            println!("sudo is installed.");
        },
        Err(_) => {
            println!("sudo is not installed. Exiting...");
            exit(1);
        }
    }
}

fn read_ips_and_ports_from_file(file_path: &str) -> Vec<String> {
    // Read IP addresses and ports from the file
    match fs::read_to_string(file_path) {
        Ok(contents) => {
            // Split the contents by lines and collect them into a vector
            contents.lines().map(|line| line.trim().to_string()).collect()
        },
        Err(_) => {
            eprintln!("Error reading file {}. Exiting...", file_path);
            exit(1);
        }
    }
}

fn main() {
    // Check and install sudo if necessary
    check_and_install_sudo();

    // Read IP addresses and ports from the file
    let ips_and_ports = read_ips_and_ports_from_file("/var/ossec/etc/ossec.conf");

    // Check if iptables is installed
    let check_iptables = Command::new("iptables")
        .output();

    match check_iptables {
        Ok(_) => {
            println!("iptables is installed.");
        },
        Err(_) => {
            println!("iptables is not installed. Exiting...");
            exit(1);
        }
    }

    // Configure iptables for strict policy
    let _ = Command::new("sudo")
        .args(&["iptables", "-P", "INPUT", "DROP"])
        .status();

    let _ = Command::new("sudo")
        .args(&["iptables", "-P", "OUTPUT", "DROP"])
        .status();

    // Add rules for each IP address and port
    for ip_port in &ips_and_ports {
        let parts: Vec<&str> = ip_port.split(':').collect();
        if parts.len() == 2 {
            let ip = parts[0];
            let port = parts[1];

            let _ = Command::new("sudo")
                .args(&["iptables", "-A", "INPUT", "-s", ip, "-p", "tcp", "--dport", port, "-j", "ACCEPT"])
                .status();

            let _ = Command::new("sudo")
                .args(&["iptables", "-A", "OUTPUT", "-d", ip, "-p", "tcp", "--sport", port, "-j", "ACCEPT"])
                .status();
        } else {
            eprintln!("Invalid format for IP and port: {}. Skipping...", ip_port);
        }
    }

    println!("iptables rules configured with strict policy based on the file /var/ossec/etc/ossec.conf.");
}
