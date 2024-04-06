use std::process::{Command, exit};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufRead, Seek, SeekFrom, Read, Write};


fn read_ip_and_port_from_file(file_path: &str) -> (String, String) {
    // Open the XML file
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    // Variables to store IP address and port
    let mut ip = String::new();
    let mut port = String::new();

    // Track if address and port are found
    let mut in_address = false;
    let mut in_port = false;

    // Iterate over each line in the file
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.contains("<address>") {
            // Extract IP address
            ip = line.trim_start_matches("      <address>").trim_end_matches("</address>").trim().to_string();
            in_address = true;
        } else if line.contains("<port>") {
            // Extract port
            port = line.trim_start_matches("      <port>").trim_end_matches("</port>").trim().to_string();
            in_port = true;
        } else if line.contains("</server>") {
            // If both address and port are found, break
            if in_address && in_port {
                break;
            }
        }
    }

    (ip, port)
}

fn update_config_file_with_timestamp(file_path: &str, timestamp: &str) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(file_path)
        .expect("Failed to open file");

    let mut content = String::new();
    file.read_to_string(&mut content).expect("Failed to read file");

    // Find the position to insert the label
    let insertion_point = content.find("</ossec_config>")
        .expect("Failed to find insertion point");

    // Insert the label with the timestamp
    let new_content = format!("\n{}<labels>\n  <label key=\"isolated.time\">{}</label>\n</labels>\n{}", &content[..insertion_point], timestamp, &content[insertion_point..]);

    // Move the cursor to the beginning of the file
    file.seek(SeekFrom::Start(0)).expect("Failed to seek to the beginning of the file");

    // Write the updated content to the file
    file.write_all(new_content.as_bytes()).expect("Failed to write to file");
}

fn main() {

    // Read IP address and port from the file
    let (ip, port) = read_ip_and_port_from_file("/var/ossec/etc/ossec.conf");

    // Get the current time as timestamp
    let current_time = chrono::Utc::now().to_rfc3339();

    // Update the configuration file with the timestamp
    update_config_file_with_timestamp("/var/ossec/etc/ossec.conf", &current_time);

    // Add your existing code to configure iptables and print the message here
    // Configure iptables for strict policy and print message
    


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

        let configure_iptables_msg = format!("iptables rules configured with strict policy based on the IP address {} and port {} from the file /var/ossec/etc/ossec.conf.", ip, port);
        println!("{}", configure_iptables_msg);

}
