<script lang="ts">
	import { onMount } from 'svelte';
	import { api, type Project, type CreateProject } from '$lib/api';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { FolderKanban, Plus, Play, Square, RotateCcw, Trash2, X, Rocket } from '@lucide/svelte';

	let projects = $state<Project[]>([]);
	let loading = $state(true);
	let showCreateDialog = $state(false);
	let newProject = $state<CreateProject>({ name: '', image: '', port: 80 });
	let creating = $state(false);
	let actionLoading = $state<string | null>(null);

	async function loadProjects() {
		loading = true;
		const result = await api.projects.list();
		if (result.data) {
			projects = result.data;
		}
		loading = false;
	}

	onMount(loadProjects);

	async function createProject() {
		if (!newProject.name) return;
		creating = true;
		const result = await api.projects.create(newProject);
		if (result.data) {
			showCreateDialog = false;
			newProject = { name: '', image: '', port: 80 };
			await loadProjects();
		} else {
			alert(result.message || 'Failed to create project');
		}
		creating = false;
	}

	async function deployProject(id: string) {
		actionLoading = id;
		const result = await api.projects.deploy(id);
		if (result.error) {
			alert(result.message || 'Failed to deploy');
		}
		await loadProjects();
		actionLoading = null;
	}

	async function stopProject(id: string) {
		actionLoading = id;
		await api.projects.stop(id);
		await loadProjects();
		actionLoading = null;
	}

	async function restartProject(id: string) {
		actionLoading = id;
		await api.projects.restart(id);
		await loadProjects();
		actionLoading = null;
	}

	async function removeProject(id: string) {
		if (!confirm('Are you sure you want to delete this project?')) return;
		actionLoading = id;
		await api.projects.remove(id);
		await loadProjects();
		actionLoading = null;
	}

	function getStatusColor(status: string): string {
		switch (status) {
			case 'running': return 'bg-green-500';
			case 'stopped': return 'bg-red-500';
			case 'deploying': return 'bg-yellow-500';
			default: return 'bg-muted-foreground';
		}
	}
</script>

<div class="space-y-6">
	<div class="flex items-center justify-between">
		<div>
			<h2 class="text-2xl font-bold tracking-tight">Projects</h2>
			<p class="text-muted-foreground">Manage your application deployments</p>
		</div>
		<Button class="gap-2" onclick={() => showCreateDialog = true}>
			<Plus class="h-4 w-4" />
			Create Project
		</Button>
	</div>

	{#if loading}
		<div class="flex items-center justify-center py-12">
			<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
		</div>
	{:else if projects.length === 0}
		<Card.Root>
			<Card.Content class="flex flex-col items-center justify-center py-16 text-center">
				<FolderKanban class="mb-4 h-12 w-12 text-muted-foreground/50" />
				<h3 class="text-lg font-semibold">No projects yet</h3>
				<p class="mb-4 text-sm text-muted-foreground">
					Create a project to deploy and manage your applications
				</p>
				<Button onclick={() => showCreateDialog = true}>Create Your First Project</Button>
			</Card.Content>
		</Card.Root>
	{:else}
		<div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
			{#each projects as project}
				<Card.Root>
					<Card.Header class="pb-2">
						<div class="flex items-center justify-between">
							<Card.Title class="text-lg">{project.name}</Card.Title>
							<span class="flex items-center gap-1.5">
								<span class="h-2 w-2 rounded-full {getStatusColor(project.status)}"></span>
								<span class="text-xs text-muted-foreground capitalize">{project.status}</span>
							</span>
						</div>
						<Card.Description>{project.description || project.slug}</Card.Description>
					</Card.Header>
					<Card.Content class="pb-2">
						<div class="text-sm space-y-1">
							{#if project.image}
								<p><span class="text-muted-foreground">Image:</span> {project.image}</p>
							{/if}
							{#if project.port}
								<p><span class="text-muted-foreground">Port:</span> {project.port}</p>
							{/if}
						</div>
					</Card.Content>
					<Card.Footer class="flex justify-end gap-2">
						{#if project.status !== 'running'}
							<Button variant="outline" size="sm" onclick={() => deployProject(project.id)} disabled={actionLoading === project.id}>
								<Rocket class="h-4 w-4 mr-1" />
								Deploy
							</Button>
						{:else}
							<Button variant="outline" size="sm" onclick={() => stopProject(project.id)} disabled={actionLoading === project.id}>
								<Square class="h-4 w-4 mr-1" />
								Stop
							</Button>
							<Button variant="outline" size="sm" onclick={() => restartProject(project.id)} disabled={actionLoading === project.id}>
								<RotateCcw class="h-4 w-4 mr-1" />
								Restart
							</Button>
						{/if}
						<Button variant="outline" size="sm" onclick={() => removeProject(project.id)} disabled={actionLoading === project.id}>
							<Trash2 class="h-4 w-4 text-destructive" />
						</Button>
					</Card.Footer>
				</Card.Root>
			{/each}
		</div>
	{/if}
</div>

<!-- Create Dialog -->
{#if showCreateDialog}
	<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
		<Card.Root class="w-full max-w-md mx-4">
			<Card.Header>
				<div class="flex items-center justify-between">
					<Card.Title>Create Project</Card.Title>
					<Button variant="ghost" size="icon" onclick={() => showCreateDialog = false}>
						<X class="h-4 w-4" />
					</Button>
				</div>
			</Card.Header>
			<Card.Content class="space-y-4">
				<div class="space-y-2">
					<Label for="name">Project Name</Label>
					<Input id="name" placeholder="my-project" bind:value={newProject.name} />
				</div>
				<div class="space-y-2">
					<Label for="description">Description (optional)</Label>
					<Input id="description" placeholder="A brief description" bind:value={newProject.description} />
				</div>
				<div class="space-y-2">
					<Label for="image">Docker Image</Label>
					<Input id="image" placeholder="nginx:latest" bind:value={newProject.image} />
				</div>
				<div class="space-y-2">
					<Label for="port">Container Port</Label>
					<Input id="port" type="number" placeholder="80" bind:value={newProject.port} />
				</div>
			</Card.Content>
			<Card.Footer class="flex justify-end gap-2">
				<Button variant="outline" onclick={() => showCreateDialog = false}>Cancel</Button>
				<Button onclick={createProject} disabled={creating || !newProject.name}>
					{creating ? 'Creating...' : 'Create Project'}
				</Button>
			</Card.Footer>
		</Card.Root>
	</div>
{/if}
