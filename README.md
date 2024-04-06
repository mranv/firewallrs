## FirewallRS

### Overview

FirewallRS is a Rust application designed to enforce strict firewall policies using `iptables` with `sudo` privileges. It ensures a secure network environment by allowing traffic only from specific IP addresses and ports while dropping all other incoming and outgoing traffic by default.

### Usage

1. **Installation and Setup**: Ensure that both `sudo` and `iptables` are installed on your system. If not found, the application automatically attempts to install them based on the detected Linux distribution (Fedora or Ubuntu).

2. **Execution**: Run the `firewallrs` executable. Upon execution, the application checks for the presence of `sudo` and `iptables`, and configures `iptables` rules accordingly.

3. **Strict Policy Enforcement**: FirewallRS enforces a strict policy where only traffic from/to a specific IP address and port is allowed. All other traffic is dropped by default.

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

- The application checks for the presence of `sudo` and `iptables`, installs them if necessary, and configures `iptables` rules.
- It ensures strict policy enforcement, allowing only specified traffic while dropping all others.
- Examples of usage and system responses are provided within the repository's README.

![Demo Image](/assets/demo.png)

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

![update ossec.conf](/assets/updateossec.conf.png)
