<script lang="ts">
  import * as Card from '$lib/components/ui/card';
  import { Badge } from '$lib/components/ui/badge';
  import { Server, Cpu, Activity, Database, Monitor } from '@lucide/svelte';
  import { Button } from '$lib/components/ui/button';
  import type { SwarmNode } from '$lib/api';
  import type { NodeListController } from '../node-list-controller.svelte';

  let { node, ctrl } = $props<{ node: SwarmNode, ctrl: NodeListController }>();

  function getStatusVariant(status: string): "default" | "destructive" | "outline" | "secondary" {
    switch (status.toLowerCase()) {
      case 'ready': return 'default';
      case 'down': return 'destructive';
      case 'disconnected': return 'secondary';
      default: return 'outline';
    }
  }

  function formatMemory(bytes: number): string {
    const gb = bytes / (1024 * 1024 * 1024);
    return gb.toFixed(2) + ' GB';
  }

  function formatCpus(nanoCpus: number): string {
    return (nanoCpus / 1e9).toFixed(1) + ' CPUs';
  }
</script>

<Card.Root>
  <Card.Header class="pb-2">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <Server class="h-5 w-5 text-primary" />
        <Card.Title class="text-lg">{node.hostname}</Card.Title>
      </div>
      <Badge variant={getStatusVariant(node.status)}>{node.status}</Badge>
    </div>
    <Card.Description class="font-mono text-xs">{node.id}</Card.Description>
  </Card.Header>
  <Card.Content>
    <div class="grid grid-cols-2 gap-4 pt-2">
      <div class="flex items-center gap-2 text-sm">
        <Activity class="h-4 w-4 text-muted-foreground" />
        <span class="capitalize">{node.role}</span>
      </div>
      <div class="flex items-center gap-2 text-sm">
        <Database class="h-4 w-4 text-muted-foreground" />
        <span>{node.addr}</span>
      </div>
      <div class="flex items-center gap-2 text-sm">
        <Cpu class="h-4 w-4 text-muted-foreground" />
        <span>{formatCpus(node.resources.nano_cpus)}</span>
      </div>
      <div class="flex items-center gap-2 text-sm">
        <div class="h-4 w-4 flex items-center justify-center font-bold text-[10px] border border-muted-foreground rounded-sm text-muted-foreground">M</div>
        <span>{formatMemory(node.resources.memory_bytes)}</span>
      </div>
    </div>
    <div class="mt-4 pt-4 border-t text-xs text-muted-foreground flex justify-between">
      <span>{node.platform}</span>
      <span>v{node.version}</span>
    </div>
    <div class="mt-4 flex gap-2">
      <Button variant="outline" size="sm" class="w-full gap-2" onclick={() => ctrl.openTerminal(node.id, node.hostname)}>
        <Monitor class="h-4 w-4" />
        Terminal
      </Button>
    </div>
  </Card.Content>
</Card.Root>
