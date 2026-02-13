# NetPulse Mini Test Version

This is a **minimal working test version** of the NetPulse network health monitoring agent. It demonstrates the core functionality of the platform without the full P2P networking and aggregation features.

## What's Implemented

### Core Measurements
- **ICMP Ping**: Measures network latency and packet loss using ICMP echo requests
- **DNS Resolution**: Tests DNS lookup performance and success rates
- **HTTP Health Checks**: Monitors HTTP endpoint availability and response times

### Privacy & Storage
- **Differential Privacy**: Applies Laplace noise to measurements (configurable epsilon)
- **SQLite Storage**: Stores all measurements locally in a SQLite database
- **Structured Data**: JSON serialization for easy querying and analysis

### Agent Features
- **Configuration**: TOML-based configuration for targets and settings
- **Periodic Scheduling**: Automatic measurement collection at configurable intervals
- **Graceful Shutdown**: Handles Ctrl+C cleanly
- **Comprehensive Logging**: Detailed logs for monitoring and debugging

## What's Not Implemented 

- **P2P Networking**: libp2p integration (planned for Phase 2)
- **Aggregation Node**: Central data collection and API (planned for Phase 3)
- **Web Dashboard**: Visualization interface (planned for Phase 4)
- **Advanced Features**: NAT traversal, peer discovery, real-time WebSocket

## Quick Start

### Prerequisites

- **Rust 1.75+** - Install from [rustup.rs](https://rustup.rs)
- **macOS/Linux** - ICMP ping requires elevated privileges on some systems

### Installation

```bash
cd /Users/kawkoiharu/code/gsoc/NetPulse/agent
cargo build --release
```

### Configuration

Edit `test-config.toml` to customize your monitoring targets:

```toml
[measurements]
interval = 30  # Measurement interval in seconds

targets = [
    "google.com",           # DNS + Ping
    "1.1.1.1",             # DNS + Ping
    "https://example.com",  # HTTP check
]

[privacy]
epsilon = 1.0  # Privacy budget (lower = more privacy)
```

### Running the Agent

```bash
# Run with default config
cd /Users/kawkoiharu/code/gsoc/NetPulse
cargo run --release --manifest-path agent/Cargo.toml -- --config test-config.toml

# Run with verbose logging
cargo run --release --manifest-path agent/Cargo.toml -- --config test-config.toml --verbose
```

**Note**: On some systems, ICMP ping requires root privileges:
```bash
sudo cargo run --release --manifest-path agent/Cargo.toml -- --config test-config.toml
```

### Viewing Measurements

Use the included example tool to view stored measurements:

```bash
cd agent
cargo run --example view_measurements
```

Or query the database directly:

```bash
sqlite3 agent.db "SELECT * FROM measurements ORDER BY timestamp DESC LIMIT 10;"
```

## Example Output

```
2026-02-13T10:45:00Z INFO Starting Network Health Monitoring Agent
2026-02-13T10:45:00Z INFO Configuration loaded successfully
2026-02-13T10:45:00Z INFO Measurement interval: 30 seconds
2026-02-13T10:45:00Z INFO Storage initialized
2026-02-13T10:45:00Z INFO Privacy layer initialized (epsilon: 1.0)
2026-02-13T10:45:00Z INFO Starting measurement loop...
2026-02-13T10:45:00Z INFO Running measurements...
2026-02-13T10:45:01Z INFO DNS google.com - 12ms (success: true)
2026-02-13T10:45:02Z INFO PING google.com - 15ms (loss: 0%)
2026-02-13T10:45:03Z INFO HTTP https://www.google.com - 145ms (status: 200)
2026-02-13T10:45:05Z INFO Measurement cycle complete
```

## Architecture

```
┌─────────────────────────────────────────┐
│         Network Health Agent            │
│                                         │
│  ┌─────────────┐    ┌───────────────┐  │
│  │ Measurements│───▶│ Privacy Layer │  │
│  │  - Ping     │    │  (Laplace)    │  │
│  │  - DNS      │    └───────┬───────┘  │
│  │  - HTTP     │            │          │
│  └─────────────┘            ▼          │
│                      ┌─────────────┐   │
│                      │   Storage   │   │
│                      │  (SQLite)   │   │
│                      └─────────────┘   │
└─────────────────────────────────────────┘
```

## Configuration File (test-config.toml)

The `test-config.toml` file controls what the agent monitors and how it behaves. It has three main sections:

### Measurements Section

```toml
[measurements]
interval = 30  # How often to run measurements (in seconds)

targets = [
    "google.com",                    # Domain name - runs DNS + Ping
    "cloudflare.com",                # Domain name - runs DNS + Ping
    "1.1.1.1",                       # IP address - runs DNS + Ping
    "8.8.8.8",                       # IP address - runs DNS + Ping
    "https://www.google.com",        # HTTP URL - runs HTTP check only
    "https://www.cloudflare.com",    # HTTP URL - runs HTTP check only
]
```

**Target Types:**
- **Domain names** (e.g., `google.com`) - Agent performs DNS resolution and ICMP ping
- **IP addresses** (e.g., `1.1.1.1`) - Agent performs DNS lookup and ICMP ping
- **HTTP/HTTPS URLs** (e.g., `https://example.com`) - Agent performs HTTP GET request only

**Customization Examples:**

Monitor your own services:
```toml
targets = [
    "github.com",
    "stackoverflow.com",
    "https://api.github.com",
]
```

Fast testing (every 10 seconds):
```toml
interval = 10
targets = ["google.com", "1.1.1.1"]
```

### Privacy Section

```toml
[privacy]
epsilon = 1.0  # Privacy budget - controls noise level
enable_geolocation_fuzzing = false  # Future feature
```

**Epsilon Values:**
- **Higher (e.g., 5.0)** - Less noise, more accurate measurements, less privacy
- **Default (1.0)** - Balanced privacy and accuracy
- **Lower (e.g., 0.1)** - More noise, less accurate measurements, more privacy

Example: If real latency is 100ms:
- `epsilon = 5.0` might measure 102ms (small noise)
- `epsilon = 1.0` might measure 95ms (medium noise)
- `epsilon = 0.1` might measure 120ms (large noise)

### Network Section

```toml
[network]
listen_addresses = ["/ip4/0.0.0.0/tcp/0"]
bootstrap_peers = []
```

These settings are for future P2P networking features (Phase 2) and are not currently used.

## Database Schema

```sql
CREATE TABLE measurements (
    id TEXT PRIMARY KEY,
    timestamp INTEGER NOT NULL,
    measurement_type TEXT NOT NULL,  -- 'Ping', 'Dns', or 'Http'
    target TEXT NOT NULL,
    result TEXT NOT NULL             -- JSON-serialized result
);
```

## Privacy Features

The agent implements **differential privacy** using the Laplace mechanism:

- **Epsilon (ε)**: Controls privacy budget (default: 1.0)
  - Lower ε = more privacy, more noise
  - Higher ε = less privacy, less noise
- **Noise Addition**: Calibrated Laplace noise added to all timing measurements
- **No PII**: Only network metrics collected, no user data

## Testing

Run unit tests:

```bash
cd agent
cargo test
```

## Troubleshooting

### "Permission denied" for ping
ICMP ping requires elevated privileges. Run with `sudo` or configure capabilities:
```bash
sudo setcap cap_net_raw+ep target/release/network-health-agent
```

### "Database is locked"
Only one agent instance can run at a time. Stop other instances before starting a new one.

### "Failed to resolve domain"
Check your internet connection and DNS settings.

## Next Steps

This mini version demonstrates the core concept. Future phases will add:

1. **Phase 2**: P2P networking with libp2p (Gossipsub, Kademlia DHT)
2. **Phase 3**: Aggregation node with REST API and PostgreSQL
3. **Phase 4**: Web dashboard with React and D3.js visualizations

## Contributing

This is a GSoC 2026 project for Internet Health Report. See the main [README.md](../README.md) for the full project roadmap.

## License

MIT OR Apache-2.0
