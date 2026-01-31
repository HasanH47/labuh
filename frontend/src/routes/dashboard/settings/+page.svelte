<script lang="ts">
	import { onMount } from 'svelte';
	import { toast } from 'svelte-sonner';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { auth, theme, activeTeam } from '$lib/stores';
	import { api, type RegistryCredential } from '$lib/api';
	import { Trash2, Plus, Container, Users } from '@lucide/svelte';

	let name = $state($auth.user?.name || '');
	let registries = $state<RegistryCredential[]>([]);
	let loadingRegistries = $state(true);
	let newRegistry = $state({
		name: '',
		team_id: '',
		registry_url: '',
		username: '',
		password: ''
	});
	let addingRegistry = $state(false);

	async function loadRegistries() {
		if (!$activeTeam?.team) {
			registries = [];
			loadingRegistries = false;
			return;
		}
		loadingRegistries = true;
		const result = await api.registries.list($activeTeam.team.id);
		if (result.data) {
			registries = result.data;
		}
		loadingRegistries = false;
	}

	onMount(() => {
		loadRegistries();
	});

	$effect(() => {
		if ($activeTeam?.team) {
			loadRegistries();
		}
	});

	async function addRegistry() {
		if (!newRegistry.name || !newRegistry.registry_url || !newRegistry.username || !newRegistry.password || !$activeTeam?.team) {
			return;
		}
		addingRegistry = true;
		const result = await api.registries.add({
			...newRegistry,
			team_id: $activeTeam.team.id
		});
		if (result.data) {
			toast.success('Registry credential added');
			registries = [result.data, ...registries];
			newRegistry = { name: '', team_id: '', registry_url: '', username: '', password: '' };
		} else {
			toast.error(result.message || result.error || 'Failed to add registry');
		}
		addingRegistry = false;
	}

	async function removeRegistry(id: string) {
		if (!$activeTeam?.team) return;
		if (!confirm('Are you sure you want to remove this registry credential?')) return;
		const result = await api.registries.remove(id, $activeTeam.team.id);
		if (!result.error) {
			toast.success('Registry credential removed');
			registries = registries.filter(r => r.id !== id);
		} else {
			toast.error(result.message || result.error);
		}
	}
</script>

<div class="space-y-6">
	<div>
		<h2 class="text-2xl font-bold tracking-tight">Settings</h2>
		<p class="text-muted-foreground">Manage your account and preferences</p>
	</div>

	<div class="grid gap-6 lg:grid-cols-2">
		<!-- Profile Settings -->
		<Card.Root>
			<Card.Header>
				<Card.Title>Profile</Card.Title>
				<Card.Description>Your account information</Card.Description>
			</Card.Header>
			<Card.Content class="space-y-4">
				<div class="space-y-2">
					<Label for="email">Email</Label>
					<Input id="email" value={$auth.user?.email || ''} disabled />
				</div>
				<div class="space-y-2">
					<Label for="name">Name</Label>
					<Input id="name" bind:value={name} placeholder="Your name" />
				</div>
				<div class="space-y-2">
					<Label>Role</Label>
					<p class="text-sm text-muted-foreground capitalize">{$auth.user?.role}</p>
				</div>
			</Card.Content>
			<Card.Footer>
				<Button>Save Changes</Button>
			</Card.Footer>
		</Card.Root>

		<!-- Appearance Settings -->
		<Card.Root class="h-fit">
			<Card.Header>
				<Card.Title>Appearance</Card.Title>
				<Card.Description>Customize the look and feel</Card.Description>
			</Card.Header>
			<Card.Content>
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Dark Mode</p>
						<p class="text-sm text-muted-foreground">Toggle between light and dark theme</p>
					</div>
					<Button variant="outline" onclick={() => theme.toggle()}>
						{$theme === 'dark' ? 'Switch to Light' : 'Switch to Dark'}
					</Button>
				</div>
			</Card.Content>
		</Card.Root>

		<!-- Container Registries -->
		<Card.Root class="lg:col-span-2">
			<Card.Header>
				<Card.Title class="flex items-center gap-2">
					<Container class="h-5 w-5" />
					Container Registries
				</Card.Title>
				<Card.Description>
					Manage credentials for private container registries (Docker Hub, GHCR, etc.)
				</Card.Description>
			</Card.Header>
			<Card.Content class="space-y-6">
				<!-- Add Registry Form -->
				<div class="grid gap-4 p-4 border rounded-lg bg-muted/30">
					<h4 class="font-medium text-sm">Add New Registry</h4>
					<div class="grid gap-4 md:grid-cols-2">
						<div class="space-y-2">
							<Label for="regName">Name (Alias)</Label>
							<Input id="regName" placeholder="My Docker Hub" bind:value={newRegistry.name} />
						</div>
						<div class="space-y-2">
							<Label for="regUrl">Registry URL</Label>
							<Input id="regUrl" placeholder="docker.io or ghcr.io" bind:value={newRegistry.registry_url} />
						</div>
						<div class="space-y-2">
							<Label for="regUser">Username</Label>
							<Input id="regUser" placeholder="username" bind:value={newRegistry.username} />
						</div>
						<div class="space-y-2">
							<Label for="regPass">Password / Token</Label>
							<Input id="regPass" type="password" placeholder="••••••••" bind:value={newRegistry.password} />
						</div>
					</div>
					<div class="flex justify-end">
						<Button onclick={addRegistry} disabled={addingRegistry || !$activeTeam?.team || $activeTeam.role === 'Viewer'}>
							<Plus class="h-4 w-4 mr-2" />
							{addingRegistry ? 'Adding...' : 'Add Credential'}
						</Button>
					</div>
				</div>

				<!-- Registry List -->
				<div>
					<h4 class="mb-4 font-medium text-sm">Saved Registries</h4>
					{#if !$activeTeam?.team}
						<div class="flex flex-col items-center justify-center py-8 text-center bg-muted/20 rounded-lg">
							<Users class="mb-2 h-8 w-8 text-muted-foreground/50" />
							<p class="text-xs text-muted-foreground">Select a team to see registries</p>
						</div>
					{:else if loadingRegistries}
						<div class="flex items-center justify-center py-8">
							<div class="animate-spin rounded-full h-6 w-6 border-b-2 border-primary"></div>
						</div>
					{:else if registries.length === 0}
						<p class="text-sm text-muted-foreground text-center py-4">
							No registry credentials saved.
						</p>
					{:else}
						<div class="space-y-2">
							{#each registries as reg}
								<div class="flex items-center justify-between p-3 rounded-md border">
									<div class="grid gap-1">
										<p class="font-medium">{reg.name}</p>
										<div class="flex items-center gap-4 text-xs text-muted-foreground">
											<span>{reg.registry_url}</span>
											<span>•</span>
											<span>{reg.username}</span>
										</div>
									</div>
									<div class="flex items-center gap-2">
										{#if $activeTeam.role !== 'Viewer'}
											<Button
												variant="ghost"
												size="icon"
												onclick={() => removeRegistry(reg.id)}
											>
												<Trash2 class="h-4 w-4 text-destructive" />
											</Button>
										{:else}
											<span class="text-[10px] text-muted-foreground italic px-2">Read Only</span>
										{/if}
									</div>
								</div>
							{/each}
						</div>
					{/if}
				</div>
			</Card.Content>
		</Card.Root>
	</div>
</div>
