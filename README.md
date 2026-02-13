# Decentralized Network Health Monitoring Platform

A privacy-preserving, decentralized platform for monitoring internet health through volunteer-run agents and peer-to-peer data sharing.

This project enables anyone to participate in internet health monitoring by running a lightweight Rust agent that:
- Performs network measurements (ping, DNS, HTTP checks)
- Preserves privacy through differential privacy
- Shares aggregated data via a P2P network (libp2p)
- Contributes to a global view of internet health

## Architecture

```
┌─────────────────┐
│ Monitoring Agent│  (Rust + libp2p)
│   - Measurements│
│   - Privacy     │
│   - P2P Network │
└────────┬────────┘
         │ Gossipsub
         ↓
┌─────────────────┐
│ Aggregation Node│  (Rust + Axum + PostgreSQL)
│   - Data Agg    │
│   - REST API    │
└────────┬────────┘
         │ HTTP/WebSocket
         ↓
┌─────────────────┐
│  Web Dashboard  │  (TypeScript + React)
│   - Heatmaps    │
│   - Analytics   │
└─────────────────┘
```

##  Project Structure

```
.
├── agent/              # Rust monitoring agent
│   ├── src/
│   │   ├── main.rs
│   │   ├── measurements/
│   │   ├── privacy/
│   │   └── p2p/
│   └── Cargo.toml
├── aggregation/        # Rust aggregation node + API
│   ├── src/
│   │   ├── main.rs
│   │   ├── api/
│   │   └── storage/
│   └── Cargo.toml
├── dashboard/          # TypeScript/React frontend
│   ├── src/
│   │   ├── components/
│   │   ├── pages/
│   │   └── App.tsx
│   └── package.json
└── docs/              # Documentation
    ├── architecture.md
    ├── api.md
    └── deployment.md
```

##  Quick Start

### Prerequisites
- Rust 1.75+ (install from [rustup.rs](https://rustup.rs))
- Node.js 20+ (for dashboard)
- PostgreSQL 15+ (for aggregation node)

### Running the Agent
```bash
cd agent
cargo run --release
```

### Running the Aggregation Node
```bash
cd aggregation
cargo run --release
```

### Running the Dashboard
```bash
cd dashboard
npm install
npm run dev
```

##  Technology Stack

### Agent
- **Rust** - Systems programming language
- **Tokio** - Async runtime
- **libp2p** - P2P networking (DHT, Gossipsub, Noise)
- **SQLite** - Local storage
- **clap** - CLI interface

### Aggregation
- **Axum** - Web framework
- **PostgreSQL + TimescaleDB** - Time-series database
- **Redis** - Caching
- **serde** - Serialization

### Dashboard
- **React 18** - UI framework
- **TypeScript** - Type safety
- **Vite** - Build tool
- **D3.js** - Data visualization
- **Leaflet** - Maps
- **Tailwind CSS** - Styling

##  Development Roadmap

### Phase 1: Agent Foundation (Weeks 1-4)
- [x] Project setup
- [ ] Basic measurement modules (ping, DNS, HTTP)
- [ ] Privacy layer (differential privacy)
- [ ] Local storage

### Phase 2: P2P Networking (Weeks 5-8)
- [ ] libp2p integration
- [ ] Peer discovery (Kademlia DHT)
- [ ] Gossipsub for data sharing
- [ ] NAT traversal

### Phase 3: Aggregation & API (Weeks 9-12)
- [ ] Aggregation node
- [ ] REST API endpoints
- [ ] Database schema
- [ ] Real-time WebSocket

### Phase 4: Dashboard (Weeks 13-16)
- [ ] React frontend
- [ ] Interactive visualizations
- [ ] Agent management
- [ ] Documentation

##  Privacy Features

- **Differential Privacy**: Calibrated noise added to measurements
- **No PII Collection**: Only network metrics, no user data
- **Local Aggregation**: Data aggregated before sharing
- **User Control**: Configurable privacy budget

