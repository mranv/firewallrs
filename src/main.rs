use std::process::{Command, exit};
use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

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
    // Open the XML file
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    // Create XML reader
    let parser = EventReader::new(reader);

    // Variables to store IP address and port
    let mut ip = String::new();
    let mut port = String::new();
    let mut in_address = false;
    let mut in_port = false;

    // Iterate over XML events
    for event in parser {
        match event {
            Ok(XmlEvent::StartElement { name, .. }) => {
                match name.local_name.as_str() {
                    "address" => in_address = true,
                    "port" => in_port = true,
                    _ => {}
                }
            }
            Ok(XmlEvent::Characters(characters)) => {
                if in_address {
                    ip = characters.to_string();
                }
                if in_port {
                    port = characters.to_string();
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                match name.local_name.as_str() {
                    "address" => in_address = false,
                    "port" => in_port = false,
                    _ => {}
                }
            }
            _ => {}
        }
    }

    (ip, port)
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
