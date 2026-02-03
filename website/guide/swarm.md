# Swarm & Clusters

Labuh v0.3.0 brings native support for **Docker Swarm**, allowing you to manage a cluster of multiple servers (nodes) directly from a single dashboard.

## Swarm Overview

Docker Swarm turns a pool of Docker hosts into a single virtual server. Labuh integrates seamlessly with Swarm to provide:

- **Multi-Node Deployment**: Deploy stacks across multiple servers.
- **High Availability**: Services can be replicated across nodes.
- **Unified Management**: Manage Managers and Workers from one UI.

## Initializing Swarm

If your server isn't part of a Swarm yet, you can initialize it directly from Labuh:

1. Go to **Nodes** sidebar menu.
2. Click **Initialize Swarm**.
3. Labuh will configure the current server as the first **Manager** node.

## Node Management

Once Swarm is active, the **Nodes** page becomes your cluster command center.

### Joining Nodes

To add more servers to your cluster:

1. Click **Join Token** on the Manager node.
2. Copy the **Worker Token** (or Manager Token).
3. On the new server (which must have Docker installed), run the provided `docker swarm join` command OR use the Labuh interface if Labuh is installed there too.

### Promoting & Demoting

- **Promote to Manager**: Gives a Worker node administrative powers.
- **Demote to Worker**: Strips administrative powers (useful for maintenance).

You can perform these actions by clicking the **...** (Options) button on any node card.

## Network Visualization

Labuh provides a powerful **Network Visualizer** to help you understand your cluster's topology.

- **Interactive Graph**: See how containers connect to networks and other containers.
- **Real-time Status**: Color-coded nodes show the health of your services.
- **Traffic Heatmap**: (Coming Soon) Visualize traffic flow between nodes.

Access the visualizer by clicking the **Network Map** tab on the Dashboard or Nodes page.
