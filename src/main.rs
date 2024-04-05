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

fn read_ip_and_port_from_file(file_path: &str) -> (String, String) {
    // Read IP address and port from the file using sudo
    let sudo_output = Command::new("sudo")
        .arg("grep")
        .arg("-oP")
        .arg("<address>.*?</address>|<port>.*?</port>")
        .arg(file_path)
        .output();

    match sudo_output {
        Ok(output) => {
            // Check if the command was successful
            if output.status.success() {
                // Convert the output bytes to a string
                let contents = String::from_utf8_lossy(&output.stdout);
                // Extract IP address and port
                let mut ip = String::new();
                let mut port = String::new();
                for line in contents.lines() {
                    if line.contains("<address>") {
                        ip = line.trim_start_matches("<address>").trim_end_matches("</address>").trim().to_string();
                    } else if line.contains("<port>") {
                        port = line.trim_start_matches("<port>").trim_end_matches("</port>").trim().to_string();
                    }
                }
                (ip, port)
            } else {
                // Print stderr if the command failed
                eprintln!("Error reading file: {}", String::from_utf8_lossy(&output.stderr));
                exit(1);
            }
        },
        Err(_) => {
            eprintln!("Error running sudo command to read file.");
            exit(1);
        }
    }
}

fn main() {
    // Check and install sudo if necessary
    check_and_install_sudo();

    // Read IP address and port from the file
    let (ip, port) = read_ip_and_port_from_file("/var/ossec/etc/ossec.conf");

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

    // Add rules for inbound and outbound traffic
    let _ = Command::new("sudo")
        .args(&["iptables", "-A", "INPUT", "-s", &ip, "-p", "tcp", "--dport", &port, "-j", "ACCEPT"])
        .status();

    let _ = Command::new("sudo")
        .args(&["iptables", "-A", "OUTPUT", "-d", &ip, "-p", "tcp", "--sport", &port, "-j", "ACCEPT"])
        .status();

    println!("iptables rules configured with strict policy based on the IP address {} and port {} from the file /var/ossec/etc/ossec.conf.", ip, port);
}
