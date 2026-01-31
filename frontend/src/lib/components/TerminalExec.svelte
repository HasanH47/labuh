<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { Terminal } from 'xterm';
	import { FitAddon } from 'xterm-addon-fit';
	import 'xterm/css/xterm.css';

	import { API_URL } from '$lib/api';
	import { browser } from '$app/environment';

	interface Props {
		containerId: string;
		onClose?: () => void;
	}

	let { containerId, onClose = () => {} }: Props = $props();

	let terminalElement: HTMLDivElement;
	let terminal: Terminal;
	let fitAddon: FitAddon;
	let socket: WebSocket;

	onMount(() => {
		terminal = new Terminal({
			cursorBlink: true,
			theme: {
				background: '#1a1a1a',
				foreground: '#f0f0f0'
			},
			fontFamily: 'JetBrains Mono, Menlo, Courier New, monospace',
			fontSize: 14
		});

		fitAddon = new FitAddon();
		terminal.loadAddon(fitAddon);
		terminal.open(terminalElement);
		fitAddon.fit();

		// Determine WebSocket URL from API_URL
		let wsUrl = API_URL.replace(/^http/, 'ws');
		const token = browser ? localStorage.getItem('token') : '';

		socket = new WebSocket(`${wsUrl}/api/containers/${containerId}/exec?token=${token}`);

		socket.onopen = () => {
			terminal.write('\r\n\x1b[32m[Connected to container terminal]\x1b[0m\r\n');
		};

		socket.onmessage = async (event) => {
			if (event.data instanceof Blob) {
				const text = await event.data.text();
				terminal.write(text);
			} else {
				terminal.write(event.data);
			}
		};

		socket.onclose = () => {
			terminal.write('\r\n\x1b[31m[Connection closed]\x1b[0m\r\n');
		};

		socket.onerror = (error) => {
			terminal.write('\r\n\x1b[31m[WebSocket Error]\x1b[0m\r\n');
			console.error('Terminal WebSocket Error:', error);
		};

		terminal.onData((data) => {
			if (socket && socket.readyState === WebSocket.OPEN) {
				socket.send(data);
			}
		});

		window.addEventListener('resize', () => fitAddon.fit());
	});

	onDestroy(() => {
		if (socket) socket.close();
		if (terminal) terminal.dispose();
	});
</script>

<div class="fixed inset-0 z-50 flex flex-col bg-black/80 p-4 backdrop-blur-sm">
	<div class="mb-2 flex items-center justify-between">
		<h3 class="font-mono text-sm text-white">Terminal: {containerId.slice(0, 12)}</h3>
		<button
			onclick={() => onClose()}
			class="rounded bg-zinc-800 px-3 py-1 text-xs text-zinc-400 hover:bg-zinc-700 hover:text-white"
		>
			Close
		</button>
	</div>
	<div bind:this={terminalElement} class="h-full w-full overflow-hidden rounded bg-[#1a1a1a] p-2"></div>
</div>

<style>
	:global(.xterm) {
		padding: 10px;
	}
</style>
