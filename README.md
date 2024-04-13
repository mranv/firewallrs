## FirewallRS

### Overview

FirewallRS is a Rust application designed to enforce strict firewall policies using `iptables` with `sudo` privileges. It ensures a secure network environment by allowing traffic only from specific IP addresses and ports while dropping all other incoming and outgoing traffic by default.

### Usage

1. **Installation and Setup**: Ensure that both `sudo` and `iptables` are installed on your system. If not found, the application automatically attempts to install them based on the detected Linux distribution (Fedora or Ubuntu).

2. **Execution**: Run the `firewallrs` executable. Upon execution, the application checks for the presence of `sudo` and `iptables`, reads IP addresses and ports from `/var/ossec/etc/ossec.conf`, and configures `iptables` rules accordingly.

3. **Strict Policy Enforcement**: FirewallRS enforces a strict policy where only traffic from/to specific IP addresses and ports specified in `/var/ossec/etc/ossec.conf` is allowed. All other traffic is dropped by default.

### Example

```bash
./target/release/firewallrs
```

### Revert back to normal stage

To revert back the iptables configuration set by the implemented code, you can execute the following commands:

```bash
sudo iptables -P INPUT ACCEPT
sudo iptables -P OUTPUT ACCEPT
sudo iptables -F
```

These commands will:

1. Set the default policy for incoming traffic (`INPUT`) and outgoing traffic (`OUTPUT`) to `ACCEPT`.
2. Flush all the rules in the iptables ruleset (`-F` option).

Executing these commands will revert the iptables configuration back to allowing all incoming and outgoing traffic by default and remove any custom rules that were added.

### Demonstration

- The application checks for the presence of `sudo` and `iptables`, installs them if necessary, and configures `iptables` rules based on the IP addresses and ports from `/var/ossec/etc/ossec.conf`.
- It ensures strict policy enforcement, allowing only specified traffic while dropping all others.
- Examples of usage and system responses are provided within the repository's README.

![Anubhav Gain](assets/demo.png)

### Reviewing the code working or not

The output of `sudo iptables -L -n` shows the current configuration of the iptables firewall rules. Here's what each part of the output means:

1. **Chain INPUT (policy DROP)**: This section shows the rules for incoming traffic. The default policy for this chain is to drop all incoming traffic (`policy DROP`), meaning unless explicitly allowed by a rule, incoming traffic is rejected.

   - The only rule in this chain allows TCP traffic (`prot opt`) from the IP address `10.0.0.2` (`source`) to any destination (`destination`) on port `1514` (`tcp dpt:1514`). This rule allows incoming traffic from `10.0.0.2` to access port `1514`.

2. **Chain FORWARD (policy ACCEPT)**: This section shows the rules for forwarding traffic. In this case, the default policy is to accept all forwarding traffic (`policy ACCEPT`), but there are no specific rules defined.

3. **Chain OUTPUT (policy DROP)**: This section shows the rules for outgoing traffic. The default policy for this chain is to drop all outgoing traffic (`policy DROP`), meaning unless explicitly allowed by a rule, outgoing traffic is rejected.

   - The only rule in this chain allows TCP traffic (`prot opt`) from any source (`source`) to the IP address `10.0.0.2` (`destination`) on port `1514` (`tcp spt:1514`). This rule allows outgoing traffic to destination IP `10.0.0.2` on port `1514`.

Based on this output, it seems like the program successfully configured iptables rules to allow incoming traffic from `10.0.0.2` on port `1514` and outgoing traffic to `10.0.0.2` on port `1514`, as expected.

![Firewall Image](assets/firewall-change.png)

### Repository

You can find the source code and additional information about FirewallRS in the [GitHub repository](https://github.com/mranv/firewallrs).

### Contributions

Contributions to FirewallRS are welcome! If you encounter any issues or have suggestions for improvement, feel free to open an issue or submit a pull request.

# Change Logs

<strong>2024-04-06T16:21:00.760527+05:30</strong>

#### What the Code Does:

The code is a Rust program designed to perform the following tasks:

1. **Check if `sudo` is installed**: The program verifies if `sudo` is installed on the system.

2. **Read IP Address and Port from XML Configuration File**: It parses the XML configuration file located at `/var/ossec/etc/ossec.conf` to extract the IP address and port.

3. **Check if `iptables` is Installed**: The program checks if `iptables` is installed on the system.

4. **Configure `iptables` Rules**: It configures `iptables` rules to enforce a strict policy on inbound and outbound traffic based on the extracted IP address and port.

5. **Update Configuration File**: The program updates the XML configuration file `/var/ossec/etc/ossec.conf` with the current timestamp for inbound and outbound traffic stop times.

#### Changes Made:

1. **Added `update_ossec_conf` Function**: This function updates the XML configuration file with inbound and outbound traffic stop times. It checks if the file already contains the required tags; if not, it appends them with the current timestamp.

2. **Modified Main Function**: Incorporated the `update_ossec_conf` function call to update the configuration file after configuring `iptables` rules.

3. **Updated Error Handling**: Improved error handling to gracefully handle potential errors during file operations and `iptables` configuration.

![update ossec.conf](assets/updateossec.conf.png)

<strong>2024-04-06T16:47:40.854216+05:30</strong>

#### Changelog

- **Added XML Parsing Functionality**: Implemented a function `read_ip_and_port_from_file` to read the IP address and port from the specified XML configuration file (`/var/ossec/etc/ossec.conf`). This function parses the XML structure using a `BufReader` and extracts the IP address and port elements.

- **Updated Configuration File with Timestamp**: Implemented a function `update_config_file_with_timestamp` to update the XML configuration file with a timestamp indicating when inbound and outbound traffic has been stopped. This function opens the file in read-write mode, finds the appropriate insertion point just before the closing `</ossec_config>` tag, and inserts a new `<labels>` section containing a `<label>` element with the specified timestamp.

- **Integrated Existing Functionality**: Integrated the existing functionality for checking and installing `sudo`, configuring iptables with strict policy, and printing a confirmation message. These functionalities are now part of the `main` function.

- **Timestamp Generation**: Utilized the `chrono` crate to generate a timestamp representing the current time in RFC 3339 format (`YYYY-MM-DDTHH:MM:SS.ffffffZ`).

- **Output Message**: A confirmation message is printed indicating that iptables rules have been configured with a strict policy based on the IP address and port extracted from the XML configuration file.

This code aims to automate the process of reading configuration details from an XML file, updating the file with relevant information, and configuring iptables accordingly, providing a streamlined solution for managing firewall rules and configuration.

![added label based saving on xml](assets/label.png)
