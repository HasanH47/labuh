<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { api, type TemplateResponse } from '$lib/api';
	import { activeTeam } from '$lib/stores';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import {
		LayoutGrid,
		Globe,
		FileText,
		Database,
		ArrowRight,
		Search,
		Users,
		Plus,
		X,
		Link as LinkIcon,
		FileCode
	} from '@lucide/svelte';
	import { toast } from 'svelte-sonner';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Textarea } from '$lib/components/ui/textarea';

	let templates = $state<TemplateResponse[]>([]);
	let loading = $state(true);
	let searchQuery = $state('');
	let showAddDialog = $state(false);
	let importMode = $state<'json' | 'url'>('url');
	let urlInput = $state('');
	let jsonInput = $state('');
	let adding = $state(false);

	async function loadTemplates() {
		loading = true;
		const result = await api.templates.list();
		if (result.data) {
			templates = result.data;
		}
		loading = false;
	}

	onMount(loadTemplates);

	const filteredTemplates = $derived(
		searchQuery
			? templates.filter(t =>
				t.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
				t.description.toLowerCase().includes(searchQuery.toLowerCase())
			)
			: templates
	);

	function getIcon(iconName: string) {
		switch (iconName) {
			case 'globe': return Globe;
			case 'file-text': return FileText;
			case 'database': return Database;
			default: return LayoutGrid;
		}
	}

	function deployTemplate(templateId: string) {
		goto(`/dashboard/stacks?template=${templateId}`);
	}

	async function handleAddTemplate() {
		adding = true;
		try {
			if (importMode === 'url') {
				if (!urlInput) return;
				const result = await api.templates.import(urlInput);
				if (result.data) {
					toast.success('Template imported successfully');
					showAddDialog = false;
					urlInput = '';
					await loadTemplates();
				} else {
					toast.error(result.message || result.error || 'Failed to import template');
				}
			} else {
				if (!jsonInput) return;
				const template = JSON.parse(jsonInput);
				const result = await api.templates.create(template);
				if (!result.error) {
					toast.success('Template added successfully');
					showAddDialog = false;
					jsonInput = '';
					await loadTemplates();
				} else {
					toast.error(result.message || result.error || 'Failed to add template');
				}
			}
		} catch (e: any) {
			toast.error(e.message || 'Error processing request');
		} finally {
			adding = false;
		}
	}

	const sampleTemplateJson = JSON.stringify({
		id: "custom-app",
		name: "Custom App",
		description: "My own custom application template",
		icon: "layout-grid",
		compose_content: "version: '3.8'\nservices:\n  app:\n    image: my-app:latest\n    ports:\n      - \"80:80\"",
		default_env: []
	}, null, 2);
</script>

<div class="space-y-6">
	<div class="flex items-center justify-between">
		<div>
			<h2 class="text-2xl font-bold tracking-tight">App Templates</h2>
			<p class="text-muted-foreground">One-click deployment for popular applications</p>
		</div>
		<Button onclick={() => showAddDialog = true} class="gap-2">
			<Plus class="h-4 w-4" />
			Add Template
		</Button>
	</div>

	<div class="relative">
		<Search class="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-muted-foreground" />
		<input
			type="text"
			placeholder="Search templates..."
			bind:value={searchQuery}
			class="w-full pl-9 pr-4 py-2 bg-background border rounded-md focus:outline-none focus:ring-2 focus:ring-ring"
		/>
	</div>

	{#if loading}
		<div class="flex items-center justify-center py-12">
			<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
		</div>
	{:else if filteredTemplates.length === 0}
		<Card.Root>
			<Card.Content class="flex flex-col items-center justify-center py-16 text-center">
				<LayoutGrid class="mb-4 h-12 w-12 text-muted-foreground/50" />
				<h3 class="text-lg font-semibold">No templates found</h3>
				<p class="text-sm text-muted-foreground">
					{searchQuery ? 'Try a different search term' : 'Templates are not available at the moment'}
				</p>
			</Card.Content>
		</Card.Root>
	{:else}
		<div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
						{#each filteredTemplates as template}
							{@const Icon = getIcon(template.icon)}
							<Card.Root class="flex flex-col h-full transition-all hover:shadow-md border-muted/60">
								<Card.Header>
									<div class="flex items-center gap-3">
										<div class="h-10 w-10 rounded-lg bg-primary/10 flex items-center justify-center text-primary">
											<Icon class="h-6 w-6" />
										</div>
										<div>
											<Card.Title>{template.name}</Card.Title>
										</div>
									</div>
								</Card.Header>
					<Card.Content class="flex-1">
						<p class="text-sm text-muted-foreground line-clamp-3">
							{template.description}
						</p>
					</Card.Content>
					<Card.Footer class="border-t bg-muted/5 py-3">
						<Button
							class="w-full gap-2"
							onclick={() => deployTemplate(template.id)}
							disabled={!$activeTeam?.team}
						>
							<Plus class="h-4 w-4" />
							Deploy
						</Button>
					</Card.Footer>
				</Card.Root>
			{/each}
		</div>
	{/if}

	{#if !$activeTeam?.team}
		<div class="bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-900/50 p-4 rounded-lg flex items-start gap-3 mt-6">
			<Users class="h-5 w-5 text-yellow-600 dark:text-yellow-500 mt-0.5" />
			<div>
				<p class="text-sm font-medium text-yellow-800 dark:text-yellow-400">Team Required</p>
				<p class="text-sm text-yellow-700 dark:text-yellow-500/80">
					You need to select a team before you can deploy templates.
					<a href="/dashboard/teams" class="font-bold underline">Go to Teams</a>
				</p>
			</div>
		</div>
	{/if}
</div>

{#if showAddDialog}
	<div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm">
		<Card.Root class="w-full max-w-2xl">
			<Card.Header class="flex flex-row items-center justify-between">
				<div>
					<Card.Title>Add New Template</Card.Title>
					<Card.Description>Import templates from a JSON URL or paste JSON content</Card.Description>
				</div>
				<Button variant="ghost" size="icon" onclick={() => showAddDialog = false}>
					<X class="h-4 w-4" />
				</Button>
			</Card.Header>
			<Card.Content class="space-y-4">
				<div class="flex p-1 bg-muted rounded-lg w-fit">
					<button
						class="px-4 py-1.5 text-sm font-medium rounded-md transition-all {importMode === 'url' ? 'bg-background shadow-sm' : 'text-muted-foreground hover:text-foreground'}"
						onclick={() => importMode = 'url'}
					>
						URL
					</button>
					<button
						class="px-4 py-1.5 text-sm font-medium rounded-md transition-all {importMode === 'json' ? 'bg-background shadow-sm' : 'text-muted-foreground hover:text-foreground'}"
						onclick={() => importMode = 'json'}
					>
						JSON Content
					</button>
				</div>

				{#if importMode === 'url'}
					<div class="space-y-2">
						<Label for="url">Template URL</Label>
						<Input
							id="url"
							placeholder="https://example.com/template.json"
							bind:value={urlInput}
						/>
						<p class="text-xs text-muted-foreground">
							The URL should return a JSON object matching the Labuh template format.
						</p>
					</div>
				{:else}
					<div class="space-y-2">
						<div class="flex items-center justify-between">
							<Label for="json">JSON Content</Label>
							<Button
								variant="ghost"
								size="sm"
								onclick={() => jsonInput = sampleTemplateJson}
							>
								Load Sample
							</Button>
						</div>
						<Textarea
							id="json"
							placeholder={sampleTemplateJson}
							bind:value={jsonInput}
							rows={12}
							class="font-mono text-sm"
						/>
					</div>
				{/if}
			</Card.Content>
			<Card.Footer class="justify-end gap-2 border-t pt-4">
				<Button variant="outline" onclick={() => showAddDialog = false}>Cancel</Button>
				<Button onclick={handleAddTemplate} disabled={adding || (importMode === 'url' ? !urlInput : !jsonInput)}>
					{adding ? 'Adding...' : 'Add Template'}
				</Button>
			</Card.Footer>
		</Card.Root>
	</div>
{/if}
