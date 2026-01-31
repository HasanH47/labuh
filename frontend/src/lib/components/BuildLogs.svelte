<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { API_URL } from '$lib/api';
	import { browser } from '$app/environment';

	interface Props {
		stackId: string;
		onClose?: () => void;
	}

	interface LogEntry {
		service: string;
		message: string;
		is_error: boolean;
	}

	let { stackId, onClose = () => {} }: Props = $props();

	let eventSource: EventSource | null = null;
	let logs = $state<LogEntry[]>([]);
	let logContainer: HTMLDivElement;

	onMount(() => {
		const token = browser ? localStorage.getItem('token') : '';
		const url = `${API_URL}/api/stacks/${stackId}/build-logs?token=${token}`;
		eventSource = new EventSource(url);

		eventSource.onmessage = (event) => {
			try {
				const data = JSON.parse(event.data);
				logs = [...logs, data];

				// Auto-scroll to bottom
				setTimeout(() => {
					if (logContainer) {
						logContainer.scrollTop = logContainer.scrollHeight;
					}
				}, 10);
			} catch (e) {
				console.error('Failed to parse build log message:', e);
			}
		};

		eventSource.onerror = (error) => {
			console.error('Build logs SSE error:', error);
            // Don't close immediately as it might try to reconnect
		};
	});

	onDestroy(() => {
		if (eventSource) {
			eventSource.close();
		}
	});
</script>

<div class="fixed inset-0 z-50 flex flex-col bg-black/80 p-4 backdrop-blur-sm">
	<div class="mb-2 flex items-center justify-between">
		<h3 class="font-mono text-sm text-white">Build Logs: {stackId}</h3>
		<button
			onclick={onClose}
			class="rounded bg-zinc-800 px-3 py-1 text-xs text-zinc-400 hover:bg-zinc-700 hover:text-white"
		>
			Close
		</button>
	</div>
	<div
		bind:this={logContainer}
		class="h-full w-full overflow-y-auto rounded bg-zinc-950 p-4 font-mono text-sm"
	>
		{#if logs.length === 0}
			<div class="text-zinc-500 italic">Waiting for logs...</div>
		{/if}
		{#each logs as log}
			<div class="mb-1 flex gap-2">
				<span class="shrink-0 text-blue-400">[{log.service}]</span>
				<span class={log.is_error ? 'text-red-400' : 'text-zinc-300'}>{log.message}</span>
			</div>
		{/each}
	</div>
</div>
