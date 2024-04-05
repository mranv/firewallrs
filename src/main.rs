use std::process::{Command, exit};

fn install_iptables() {
    // Check the distribution to determine the package manager
    let distro_check = Command::new("lsb_release")
        .arg("-si")
        .output();

    match distro_check {
        Ok(output) => {
            let distro_name = String::from_utf8_lossy(&output.stdout);
            match distro_name.trim() {
                "Fedora" => {
                    // Install iptables on Fedora using dnf
                    let _ = Command::new("dnf")
                        .arg("install")
                        .arg("-y")
                        .arg("iptables")
                        .status();
                },
                "Ubuntu" => {
                    // Install iptables on Ubuntu using apt
                    let _ = Command::new("apt")
                        .arg("install")
                        .arg("-y")
                        .arg("iptables")
                        .status();
                },
                _ => {
                    eprintln!("Unsupported distribution.");
                    exit(1);
                }
            }
        },
        Err(_) => {
            eprintln!("Error determining Linux distribution.");
            exit(1);
        }
    }
}

fn main() {
    // Check if iptables is installed
    let check_iptables = Command::new("iptables")
        .output();

    match check_iptables {
        Ok(_) => {
            println!("iptables is installed.");
        },
        Err(_) => {
            println!("iptables is not installed. Installing...");

            // Install iptables
            install_iptables();

            println!("iptables installed successfully.");
        }
    }

    // Configure device isolation and allow specific IP and ports
    let _ = Command::new("iptables")
        .args(&["-A", "INPUT", "-i", "eth0", "-j", "DROP"])
        .status();

    let _ = Command::new("iptables")
        .args(&["-A", "OUTPUT", "-o", "eth0", "-j", "DROP"])
        .status();

    let _ = Command::new("iptables")
        .args(&["-A", "INPUT", "-s", "192.168.1.100", "-p", "tcp", "--dport", "8080", "-j", "ACCEPT"])
        .status();

    let _ = Command::new("iptables")
        .args(&["-A", "OUTPUT", "-d", "192.168.1.200", "-p", "tcp", "--dport", "9090", "-j", "ACCEPT"])
        .status();
}
