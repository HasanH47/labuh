<script lang="ts">
  import * as Card from '$lib/components/ui/card';
  import { Button } from '$lib/components/ui/button';
  import { AlertTriangle, Trash2 } from '@lucide/svelte';
  import { activeTeam } from '$lib/stores';
  import ConfirmationDialog from '$lib/components/ConfirmationDialog.svelte';
  import { TeamController } from '$lib/features/teams/team-controller.svelte';

  const teamCtrl = new TeamController();
  const isOwner = $derived($activeTeam?.role === 'Owner');
</script>

<Card.Root class="lg:col-span-2 border-destructive/50">
  <Card.Header>
    <Card.Title class="flex items-center gap-2 text-destructive">
      <AlertTriangle class="h-5 w-5" />
      Danger Zone
    </Card.Title>
    <Card.Description>
      Irreversible actions for the team <strong>{$activeTeam?.team?.name}</strong>
    </Card.Description>
  </Card.Header>
  <Card.Content class="space-y-4">
    <div class="flex items-center justify-between p-4 border border-destructive/20 rounded-lg bg-destructive/10">
      <div>
        <h4 class="font-medium text-destructive">Delete Team</h4>
        <p class="text-sm text-muted-foreground">
          Permanently delete this team and all its resources (stacks, containers, volumes).
        </p>
      </div>
      <Button
        variant="destructive"
        disabled={!isOwner || !$activeTeam?.team}
        onclick={() => $activeTeam?.team && teamCtrl.requestDeleteTeam($activeTeam.team.id)}
      >
        <Trash2 class="h-4 w-4 mr-2" />
        Delete Team
      </Button>
    </div>
    {#if !isOwner}
      <p class="text-xs text-muted-foreground italic text-center">
        Only team owners can perform these actions.
      </p>
    {/if}
  </Card.Content>
</Card.Root>

<ConfirmationDialog
  bind:open={teamCtrl.showDeleteTeamConfirm}
  title="Delete Team"
  description="Are you sure you want to delete this team? This action cannot be undone and will delete all stacks and resources within the team."
  confirmText="Delete Team"
  variant="destructive"
  onConfirm={() => teamCtrl.confirmDeleteTeam()}
/>
