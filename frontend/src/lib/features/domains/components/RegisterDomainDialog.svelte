<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import * as Select from '$lib/components/ui/select';
	import { X, Globe, Plus, Loader2 } from '@lucide/svelte';
	import type { DomainController } from '../domain-controller.svelte';

	let { ctrl } = $props<{ ctrl: DomainController }>();

	function handleProviderChange(val: string) {
		if (val) {
			ctrl.selectedProvider = val;
			ctrl.fetchAvailableDomains(val);
		}
	}

	function handleStackChange(val: string) {
		if (val) {
			ctrl.selectedStackId = val;
			ctrl.fetchContainers(val);
		}
	}

    function handleContainerChange(val: string) {
        if (val) ctrl.selectedContainer = val;
    }
</script>

{#if ctrl.showRegisterDialog}
<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 overflow-y-auto py-8">
    <Card.Root class="w-full max-w-lg mx-4">
        <Card.Header>
            <div class="flex items-center justify-between">
                <Card.Title class="flex items-center gap-2">
                    <Globe class="h-5 w-5" />
                    Register & Attach Domain
                </Card.Title>
                <Button variant="ghost" size="icon" onclick={() => ctrl.showRegisterDialog = false}>
                    <X class="h-4 w-4" />
                </Button>
            </div>
            <Card.Description>
                Configure a new domain/subdomain and attach it to a running stack service.
            </Card.Description>
        </Card.Header>

        <Card.Content class="space-y-4">
            <div class="space-y-4">
                <Label>Exposure Type</Label>
                <div class="flex items-center gap-2 p-1 bg-muted rounded-lg">
                    <button
                        class="flex-1 py-1.5 text-sm font-medium rounded-md transition-all {ctrl.selectedType === 'Caddy' ? 'bg-background shadow-sm text-foreground' : 'text-muted-foreground hover:text-foreground'}"
                        onclick={() => ctrl.selectedType = 'Caddy'}
                    >
                        Public (Caddy)
                    </button>
                    <button
                        class="flex-1 py-1.5 text-sm font-medium rounded-md transition-all {ctrl.selectedType === 'Tunnel' ? 'bg-background shadow-sm text-foreground' : 'text-muted-foreground hover:text-foreground'}"
                        onclick={() => ctrl.selectedType = 'Tunnel'}
                    >
                        Secure Tunnel
                    </button>
                </div>
            </div>

            {#if ctrl.selectedType === 'Tunnel'}
                <div class="space-y-4 p-3 border rounded-md bg-blue-500/5 border-blue-500/20">
                    <div class="space-y-2">
                        <Label for="tunnel_id">Tunnel ID</Label>
                        <Input id="tunnel_id" placeholder="xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx" bind:value={ctrl.tunnelId} />
                        <p class="text-[10px] text-muted-foreground">
                            Optional if token provided. Leave empty to auto-extract from token.
                        </p>
                    </div>
                    <div class="space-y-2">
                        <Label for="tunnel_token">Tunnel Token</Label>
                        <Input id="tunnel_token" placeholder="eyJh..." bind:value={ctrl.tunnelToken} />
                        <p class="text-[10px] text-muted-foreground">
                            Optional if ID provided. Required to automatically start the tunnel agent.
                        </p>
                    </div>
                    <div class="p-2 bg-blue-500/10 rounded border border-blue-500/20 text-[10px] text-blue-600 dark:text-blue-400">
                        <strong>Note:</strong> At least one of the above is required to identify your secure tunnel.
                    </div>
                </div>
            {/if}

            <div class="grid grid-cols-2 gap-4">
                <div class="space-y-2">
                    <Label>DNS Provider</Label>
                    <Select.Root type="single" value={ctrl.selectedProvider} onValueChange={handleProviderChange}>
                        <Select.Trigger class="w-full">
                            {ctrl.selectedProvider}
                        </Select.Trigger>
                        <Select.Content>
                            <Select.Item value="Custom">Custom (Manual)</Select.Item>
                            {#each ctrl.dnsConfigs as config}
                                <Select.Item value={config.provider}>{config.provider}</Select.Item>
                            {/each}
                        </Select.Content>
                    </Select.Root>
                </div>

                <div class="space-y-2">
                    <Label>Base Domain</Label>
                    {#if ctrl.selectedProvider === 'Custom' || ctrl.availableBaseDomains.length === 0}
                        <Input placeholder="example.com" bind:value={ctrl.selectedBaseDomain} />
                    {:else}
                        <Select.Root type="single" value={ctrl.selectedBaseDomain} onValueChange={(v) => v && (ctrl.selectedBaseDomain = v)}>
                            <Select.Trigger class="w-full">
                                {ctrl.selectedBaseDomain || "Select domain"}
                            </Select.Trigger>
                            <Select.Content>
                                {#each ctrl.availableBaseDomains as d}
                                    <Select.Item value={d}>{d}</Select.Item>
                                {/each}
                            </Select.Content>
                        </Select.Root>
                    {/if}
                </div>
            </div>

            <div class="space-y-2">
                <Label for="subdomain">Subdomain (Optional)</Label>
                <div class="flex items-center gap-2">
                    <Input id="subdomain" placeholder="app" bind:value={ctrl.subdomain} class="text-right" />
                    <span class="text-muted-foreground">.{ctrl.selectedBaseDomain || "domain.com"}</span>
                </div>
            </div>

            <div class="flex items-center gap-2">
                <input type="checkbox" id="advanced_dns" class="h-4 w-4 rounded border-gray-300" bind:checked={ctrl.isAdvancedDns} />
                <Label for="advanced_dns" class="font-normal cursor-pointer">Advanced DNS Configuration</Label>
            </div>

            {#if ctrl.isAdvancedDns}
                 <div class="grid grid-cols-3 gap-4 p-3 border rounded-md bg-muted/30">
                    <div class="col-span-1 space-y-2">
                        <Label>Type</Label>
                        <Select.Root type="single" value={ctrl.dnsRecordType} onValueChange={(v) => v && (ctrl.dnsRecordType = v)}>
                            <Select.Trigger>
                                {ctrl.dnsRecordType}
                            </Select.Trigger>
                            <Select.Content>
                                <Select.Item value="A">A</Select.Item>
                                <Select.Item value="CNAME">CNAME</Select.Item>
                                <Select.Item value="TXT">TXT</Select.Item>
                                <Select.Item value="MX">MX</Select.Item>
                            </Select.Content>
                        </Select.Root>
                    </div>
                    <div class="col-span-2 space-y-2">
                        <Label>Content</Label>
                        <Input placeholder="1.2.3.4" bind:value={ctrl.dnsRecordContent} />
                    </div>
                 </div>
            {/if}

            {#if ctrl.selectedProvider === 'Cloudflare'}
                <div class="flex items-center gap-2 p-3 border rounded-md bg-orange-500/5 border-orange-500/20">
                    <input type="checkbox" id="proxied" class="h-4 w-4 rounded border-gray-300" bind:checked={ctrl.proxied} />
                    <div class="flex flex-col">
                        <Label for="proxied" class="font-normal cursor-pointer text-orange-600 dark:text-orange-400 font-medium text-sm">Proxied (Cloudflare)</Label>
                        <p class="text-[10px] text-muted-foreground">Accelerate and protect your site with Cloudflare's proxy.</p>
                    </div>
                </div>
            {/if}

            <hr class="my-4 border-dashed" />

            <div class="grid grid-cols-2 gap-4">
                <div class="space-y-2">
                    <Label>Assign to Stack</Label>
                    <Select.Root type="single" value={ctrl.selectedStackId} onValueChange={handleStackChange}>
                        <Select.Trigger class="w-full">
                            {ctrl.stacks.find((s: any) => s.id === ctrl.selectedStackId)?.name || "Select Stack"}
                        </Select.Trigger>
                        <Select.Content>
                            {#each ctrl.stacks as stack}
                                <Select.Item value={stack.id}>{stack.name}</Select.Item>
                            {/each}
                        </Select.Content>
                    </Select.Root>
                </div>

                <div class="space-y-2">
                    <Label>Service/Container</Label>
                    <Select.Root type="single" value={ctrl.selectedContainer} onValueChange={handleContainerChange}>
                        <Select.Trigger class="w-full">
                            {ctrl.selectedContainer || "Select Service"}
                        </Select.Trigger>
                        <Select.Content>
                            {#each ctrl.containers as c}
                                {#each c.names as n}
                                    <Select.Item value={n.replace('/', '')}>{n.replace('/', '')}</Select.Item>
                                {/each}
                            {/each}
                        </Select.Content>
                    </Select.Root>
                </div>
            </div>

            <div class="space-y-2">
                <Label for="port">Container Port</Label>
                <Input id="port" type="number" bind:value={ctrl.selectedPort} />
                <p class="text-[10px] text-muted-foreground">The internal port your service is listening on (default: 80).</p>
            </div>
        </Card.Content>

        <Card.Footer class="flex justify-end gap-2">
            <Button variant="outline" onclick={() => ctrl.showRegisterDialog = false}>Cancel</Button>
            <Button disabled={ctrl.registrationLoading} onclick={() => ctrl.registerDomain()}>
                {#if ctrl.registrationLoading}
                    <Loader2 class="mr-2 h-4 w-4 animate-spin" />
                    Registering...
                {:else}
                    Register Domain
                {/if}
            </Button>
        </Card.Footer>
    </Card.Root>
</div>
{/if}
