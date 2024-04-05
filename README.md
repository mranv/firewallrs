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

### Demonstration

- The application checks for the presence of `sudo` and `iptables`, installs them if necessary, and configures `iptables` rules.
- It ensures strict policy enforcement, allowing only specified traffic while dropping all others.
- Examples of usage and system responses are provided within the repository's README.

### Repository

You can find the source code and additional information about FirewallRS in the [GitHub repository](https://github.com/mranv/firewallrs).

### Contributions

Contributions to FirewallRS are welcome! If you encounter any issues or have suggestions for improvement, feel free to open an issue or submit a pull request.

### License

This project is licensed under the [MIT License](LICENSE).
