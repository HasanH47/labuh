<script lang="ts">
  import * as Card from '$lib/components/ui/card';
  import { Button } from '$lib/components/ui/button';
  import { Input } from '$lib/components/ui/input';
  import { Label } from '$lib/components/ui/label';
  import { X, Ship, Loader2, Copy, ShieldAlert } from '@lucide/svelte';
  import type { NodeListController } from '../node-list-controller.svelte';
  import { toast } from 'svelte-sonner';

  let { ctrl } = $props<{ ctrl: NodeListController }>();

  function copy(text: string) {
    navigator.clipboard.writeText(text);
    toast.success("Copied to clipboard");
  }
</script>

{#if ctrl.showInitDialog}
  <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
    <Card.Root class="w-full max-w-md mx-4">
      <Card.Header>
        <div class="flex items-center justify-between">
          <Card.Title class="flex items-center gap-2">
            <Ship class="h-5 w-5" />
            Initialize Swarm
          </Card.Title>
          <Button variant="ghost" size="icon" onclick={() => ctrl.showInitDialog = false}>
            <X class="h-4 w-4" />
          </Button>
        </div>
        <Card.Description>
          Initialize this node as a Swarm Manager.
        </Card.Description>
      </Card.Header>
      <Card.Content class="space-y-4">
        <div class="space-y-2">
          <Label for="listen_addr">Listen Address</Label>
          <Input id="listen_addr" bind:value={ctrl.listenAddr} placeholder="0.0.0.0:2377" />
          <p class="text-xs text-muted-foreground">The address where the swarm manager will listen for connections.</p>
        </div>
      </Card.Content>
      <Card.Footer class="flex justify-end gap-2">
        <Button variant="outline" onclick={() => ctrl.showInitDialog = false}>Cancel</Button>
        <Button onclick={() => ctrl.initSwarm()} disabled={ctrl.loadingAction}>
          {#if ctrl.loadingAction}
            <Loader2 class="mr-2 h-4 w-4 animate-spin" />
            Initializing...
          {:else}
            Initialize
          {/if}
        </Button>
      </Card.Footer>
    </Card.Root>
  </div>
{/if}

{#if ctrl.showJoinDialog}
  <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
    <Card.Root class="w-full max-w-md mx-4">
      <Card.Header>
        <div class="flex items-center justify-between">
          <Card.Title class="flex items-center gap-2">
            <Ship class="h-5 w-5" />
            Join Swarm
          </Card.Title>
          <Button variant="ghost" size="icon" onclick={() => ctrl.showJoinDialog = false}>
            <X class="h-4 w-4" />
          </Button>
        </div>
        <Card.Description>
          Join an existing Swarm cluster.
        </Card.Description>
      </Card.Header>
      <Card.Content class="space-y-4">
        <div class="space-y-2">
          <Label for="join_listen_addr">Local Listen Address</Label>
          <Input id="join_listen_addr" bind:value={ctrl.listenAddr} placeholder="0.0.0.0:2377" />
        </div>
        <div class="space-y-2">
          <Label for="remote_addr">Manager Address</Label>
          <Input id="remote_addr" bind:value={ctrl.remoteAddr} placeholder="manager-ip:2377" />
        </div>
        <div class="space-y-2">
          <Label for="join_token">Join Token</Label>
          <Input id="join_token" bind:value={ctrl.joinToken} placeholder="SWMTKN-..." />
        </div>
      </Card.Content>
      <Card.Footer class="flex justify-end gap-2">
        <Button variant="outline" onclick={() => ctrl.showJoinDialog = false}>Cancel</Button>
        <Button onclick={() => ctrl.joinSwarm()} disabled={ctrl.loadingAction}>
          {#if ctrl.loadingAction}
            <Loader2 class="mr-2 h-4 w-4 animate-spin" />
            Joining...
          {:else}
            Join Swarm
          {/if}
        </Button>
      </Card.Footer>
    </Card.Root>
  </div>
{/if}

{#if ctrl.showTokensDialog && ctrl.swarmTokens}
  <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
    <Card.Root class="w-full max-w-lg mx-4">
      <Card.Header>
        <div class="flex items-center justify-between">
          <Card.Title class="flex items-center gap-2">
            <ShieldAlert class="h-5 w-5 text-yellow-500" />
            Swarm Join Tokens
          </Card.Title>
          <Button variant="ghost" size="icon" onclick={() => ctrl.showTokensDialog = false}>
            <X class="h-4 w-4" />
          </Button>
        </div>
        <Card.Description>
          Use these tokens to add more nodes to your cluster.
        </Card.Description>
      </Card.Header>
      <Card.Content class="space-y-6">
        <div class="space-y-2">
          <div class="flex items-center justify-between">
            <Label>Manager Join Token</Label>
            <Button variant="ghost" size="sm" class="h-8 gap-1" onclick={() => copy(ctrl.swarmTokens?.manager || '')}>
              <Copy class="h-3 w-3" />
              Copy
            </Button>
          </div>
          <div class="p-2 bg-muted rounded border text-xs break-all font-mono">
            {ctrl.swarmTokens.manager}
          </div>
        </div>

        <div class="space-y-2">
          <div class="flex items-center justify-between">
            <Label>Worker Join Token</Label>
            <Button variant="ghost" size="sm" class="h-8 gap-1" onclick={() => copy(ctrl.swarmTokens?.worker || '')}>
              <Copy class="h-3 w-3" />
              Copy
            </Button>
          </div>
          <div class="p-2 bg-muted rounded border text-xs break-all font-mono">
            {ctrl.swarmTokens.worker}
          </div>
        </div>

        <div class="p-3 border rounded-md bg-blue-500/5 border-blue-500/20 text-xs">
          <p class="font-medium text-blue-600 dark:text-blue-400 mb-1">How to join?</p>
          <p class="text-muted-foreground">Run the following command on the node you want to add:</p>
          <code class="block mt-2 p-2 bg-muted rounded font-mono text-[10px]">
            docker swarm join --token {'<token>'} {'<manager-ip>'}:2377
          </code>
        </div>
      </Card.Content>
      <Card.Footer class="flex justify-end">
        <Button onclick={() => ctrl.showTokensDialog = false}>Close</Button>
      </Card.Footer>
    </Card.Root>
  </div>
{/if}
