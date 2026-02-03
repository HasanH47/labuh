<script lang="ts">
  import { onMount } from 'svelte';
  import { NodeListController } from '$lib/features/nodes/node-list-controller.svelte';
  import NodeCard from '$lib/features/nodes/components/NodeCard.svelte';
  import { Button } from '$lib/components/ui/button';
  import { RefreshCw, ShieldAlert, Ship, Plus } from '@lucide/svelte';
  import SwarmActionDialogs from '$lib/features/nodes/components/SwarmActionDialogs.svelte';
  import NodeTerminal from '$lib/features/nodes/components/NodeTerminal.svelte';

  const ctrl = new NodeListController();

  onMount(() => {
    ctrl.init();
  });
</script>

<div class="space-y-6">
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-3xl font-bold tracking-tight">Nodes</h1>
      <p class="text-muted-foreground">
        Manage your Docker Swarm cluster nodes and resource distribution.
      </p>
    </div>
    <div class="flex items-center gap-2">
      {#if !ctrl.isSwarmEnabled}
        <Button size="sm" class="gap-2" onclick={() => ctrl.showInitDialog = true}>
          <Ship class="h-4 w-4" />
          Initialize Swarm
        </Button>
        <Button variant="outline" size="sm" class="gap-2" onclick={() => ctrl.showJoinDialog = true}>
          <Plus class="h-4 w-4" />
          Join Swarm
        </Button>
      {:else}
        <Button variant="outline" size="sm" class="gap-2" onclick={() => ctrl.loadTokens()}>
          <ShieldAlert class="h-4 w-4" />
          Join Tokens
        </Button>
      {/if}
      <Button variant="outline" size="sm" onclick={() => ctrl.loadNodes()} disabled={ctrl.loading || !ctrl.isSwarmEnabled}>
        <RefreshCw class="mr-2 h-4 w-4 {ctrl.loading ? 'animate-spin' : ''}" />
        Refresh
      </Button>
    </div>
  </div>

  {#if ctrl.loading}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      {#each Array(3) as _}
        <div class="h-[200px] rounded-xl border bg-card animate-pulse"></div>
      {/each}
    </div>
  {:else if !ctrl.isSwarmEnabled}
    <div class="flex flex-col items-center justify-center p-12 border rounded-xl bg-muted/30 text-center space-y-4">
      <div class="p-4 rounded-full bg-yellow-500/10 text-yellow-500">
        <ShieldAlert class="h-12 w-12" />
      </div>
      <div class="space-y-2 max-w-md">
        <h2 class="text-xl font-semibold">Swarm Mode Disabled</h2>
        <p class="text-muted-foreground">
          Docker Swarm is not enabled on this host. Multi-node features are only available when running in Swarm mode.
        </p>
      </div>
      <Button variant="outline" href="https://docs.docker.com/engine/swarm/" target="_blank">
        Learn about Docker Swarm
      </Button>
    </div>
  {:else if ctrl.nodes.length === 0}
    <div class="flex flex-col items-center justify-center p-12 border rounded-xl bg-muted/30 text-center space-y-4">
      <div class="p-4 rounded-full bg-primary/10 text-primary">
        <Ship class="h-12 w-12" />
      </div>
      <div class="space-y-2 max-w-md">
        <h2 class="text-xl font-semibold">No Nodes Found</h2>
        <p class="text-muted-foreground">
          We couldn't find any nodes in your Swarm cluster. This is unexpected for an active cluster.
        </p>
      </div>
      <Button variant="outline" onclick={() => ctrl.loadNodes()}>Try Again</Button>
    </div>
  {:else}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      {#each ctrl.nodes as node}
        <NodeCard {node} {ctrl} />
      {/each}
    </div>
  {/if}
</div>

<SwarmActionDialogs {ctrl} />

{#if ctrl.showTerminal && ctrl.selectedNodeId}
  <NodeTerminal
    nodeId={ctrl.selectedNodeId}
    nodeName={ctrl.selectedNodeName}
    onClose={() => ctrl.showTerminal = false}
  />
{/if}
