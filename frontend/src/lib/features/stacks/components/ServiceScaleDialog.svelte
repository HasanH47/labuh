<script lang="ts">
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';

	let { open = $bindable(), serviceName, initialReplicas = 1, onConfirm } = $props();

	let replicas = $state(1);

	$effect(() => {
		if (open) {
			replicas = initialReplicas;
		}
	});

	function handleConfirm() {
		onConfirm(replicas);
		open = false;
	}
</script>

<AlertDialog.Root bind:open>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>Scale Service: {serviceName}</AlertDialog.Title>
			<AlertDialog.Description>
				Update the number of replicas for this service. Docker Swarm will balance instances across the
				cluster.
			</AlertDialog.Description>
		</AlertDialog.Header>

		<div class="grid gap-4 py-4">
			<div class="grid grid-cols-4 items-center gap-4">
				<Label for="replicas" class="text-right">Replicas</Label>
				<Input id="replicas" type="number" min="0" bind:value={replicas} class="col-span-3" />
			</div>
		</div>

		<AlertDialog.Footer>
			<AlertDialog.Cancel onclick={() => (open = false)}>Cancel</AlertDialog.Cancel>
			<AlertDialog.Action onclick={handleConfirm}>Update Scale</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>
