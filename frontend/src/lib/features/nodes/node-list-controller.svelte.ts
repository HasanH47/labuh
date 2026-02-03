import { api, type SwarmNode } from "$lib/api";
import { toast } from "svelte-sonner";

export class NodeListController {
  nodes = $state<SwarmNode[]>([]);
  loading = $state(true);
  isSwarmEnabled = $state(false);
  swarmTokens = $state<{ manager: string; worker: string } | null>(null);

  // Dialog states
  showInitDialog = $state(false);
  showJoinDialog = $state(false);
  showTokensDialog = $state(false);
  showTerminal = $state(false);
  selectedNodeId = $state<string | null>(null);
  selectedNodeName = $state<string | null>(null);

  listenAddr = $state("0.0.0.0:2377");
  remoteAddr = $state("");
  joinToken = $state("");
  loadingAction = $state(false);

  async init() {
    await this.checkSwarm();
    if (this.isSwarmEnabled) {
      await this.loadNodes();
    } else {
      this.loading = false;
    }
  }

  async checkSwarm() {
    const result = await api.nodes.swarm();
    if (result.data) {
      this.isSwarmEnabled = result.data.enabled;
    } else {
      this.isSwarmEnabled = false;
    }
  }

  async loadNodes() {
    this.loading = true;
    const result = await api.nodes.list();
    if (result.data) {
      this.nodes = result.data;
    } else {
      toast.error(result.message || result.error || "Failed to load nodes");
    }
    this.loading = false;
  }

  async initSwarm() {
    this.loadingAction = true;
    try {
      const res = await api.nodes.initSwarm(this.listenAddr);
      if (res.data) {
        toast.success("Swarm initialized successfully!");
        this.showInitDialog = false;
        await this.init();
      } else {
        toast.error(res.error || "Failed to initialize Swarm");
      }
    } finally {
      this.loadingAction = false;
    }
  }

  async joinSwarm() {
    this.loadingAction = true;
    try {
      const res = await api.nodes.joinSwarm({
        listen_addr: this.listenAddr,
        remote_addr: this.remoteAddr,
        token: this.joinToken,
      });
      if (res.data) {
        toast.success("Joined Swarm successfully!");
        this.showJoinDialog = false;
        await this.init();
      } else {
        toast.error(res.error || "Failed to join Swarm");
      }
    } finally {
      this.loadingAction = false;
    }
  }

  async loadTokens() {
    const res = await api.nodes.getSwarmTokens();
    if (res.data) {
      this.swarmTokens = res.data;
      this.showTokensDialog = true;
    } else {
      toast.error("Failed to fetch Swarm tokens");
    }
  }

  openTerminal(id: string, name: string) {
    this.selectedNodeId = id;
    this.selectedNodeName = name;
    this.showTerminal = true;
  }
}
