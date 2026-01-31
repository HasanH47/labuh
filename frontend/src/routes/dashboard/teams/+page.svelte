<script lang="ts">
	import { onMount } from 'svelte';
	import { toast } from 'svelte-sonner';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import * as Select from '$lib/components/ui/select';
	import { api, type TeamResponse, type TeamMember, type TeamRole } from '$lib/api';
	import { activeTeam } from '$lib/stores';
	import { Users, Plus, Trash2, Shield, UserPlus, Star } from '@lucide/svelte';

	let teams = $state<TeamResponse[]>([]);
	let loadingTeams = $state(true);
	let newTeamName = $state('');
	let creatingTeam = $state(false);

	let selectedTeamMembers = $state<TeamMember[]>([]);
	let loadingMembers = $state(false);
	let inviteEmail = $state('');
	let inviteRole = $state<TeamRole>('Developer');
	let inviting = $state(false);

	async function loadTeams() {
		loadingTeams = true;
		const result = await api.teams.list();
		if (result.data) {
			teams = result.data;
			// If no active team is set, set the first one as active
			if (!$activeTeam && teams.length > 0) {
				activeTeam.setActiveTeam(teams[0]);
			}
		}
		loadingTeams = false;
	}

	async function createTeam() {
		if (!newTeamName) return;
		creatingTeam = true;
		const result = await api.teams.create(newTeamName);
		if (result.data) {
			toast.success(`Team "${newTeamName}" created`);
			newTeamName = '';
			await loadTeams();
		} else {
			toast.error(result.message || result.error || 'Failed to create team');
		}
		creatingTeam = false;
	}

	async function loadMembers(teamId: string) {
		loadingMembers = true;
		const result = await api.teams.listMembers(teamId);
		if (result.data) {
			selectedTeamMembers = result.data;
		}
		loadingMembers = false;
	}

	async function inviteMember() {
		if (!$activeTeam?.team || !inviteEmail) return;
		inviting = true;
		const result = await api.teams.addMember($activeTeam.team.id, inviteEmail, inviteRole);
		if (!result.error) {
			toast.success(`Invited ${inviteEmail}`);
			inviteEmail = '';
			await loadMembers($activeTeam.team.id);
		} else {
			toast.error(result.message || result.error);
		}
		inviting = false;
	}

	async function removeMember(userId: string) {
		if (!$activeTeam?.team) return;
		if (!confirm('Are you sure you want to remove this member?')) return;

		const result = await api.teams.removeMember($activeTeam.team.id, userId);
		if (!result.error) {
			toast.success('Member removed');
			await loadMembers($activeTeam.team.id);
		} else {
			toast.error(result.message || result.error);
		}
	}

	async function updateRole(userId: string, role: TeamRole) {
		if (!$activeTeam?.team) return;
		const result = await api.teams.updateMemberRole($activeTeam.team.id, userId, role);
		if (!result.error) {
			toast.success('Role updated');
			await loadMembers($activeTeam.team.id);
		} else {
			toast.error(result.message || result.error);
		}
	}

	onMount(() => {
		loadTeams();
	});

	$effect(() => {
		if ($activeTeam?.team) {
			loadMembers($activeTeam.team.id);
		}
	});

	const roles: TeamRole[] = ['Owner', 'Admin', 'Developer', 'Viewer'];
</script>

<div class="space-y-6">
	<div>
		<h2 class="text-2xl font-bold tracking-tight">Teams</h2>
		<p class="text-muted-foreground">Manage your teams and collaborations</p>
	</div>

	<div class="grid gap-6 lg:grid-cols-3">
		<!-- Teams List -->
		<Card.Root class="lg:col-span-1 h-fit">
			<Card.Header>
				<Card.Title class="flex items-center gap-2">
					<Users class="h-5 w-5" />
					Your Teams
				</Card.Title>
				<Card.Description>Teams you are a member of</Card.Description>
			</Card.Header>
			<Card.Content class="space-y-4">
				<div class="space-y-2">
					<div class="flex gap-2">
						<Input placeholder="Team name" bind:value={newTeamName} />
						<Button size="icon" onclick={createTeam} disabled={creatingTeam || !newTeamName}>
							<Plus class="h-4 w-4" />
						</Button>
					</div>
				</div>

				<div class="space-y-1">
					{#if loadingTeams}
						<div class="flex justify-center p-4">
							<div class="animate-spin rounded-full h-5 w-5 border-b-2 border-primary"></div>
						</div>
					{:else if teams.length === 0}
						<p class="text-xs text-center text-muted-foreground py-4">No teams found.</p>
					{:else}
						{#each teams as t}
							<button
								onclick={() => activeTeam.setActiveTeam(t)}
								class="flex w-full items-center justify-between p-2 text-sm rounded-md transition-colors hover:bg-muted {$activeTeam?.team?.id === t.team?.id ? 'bg-primary/10 text-primary font-medium' : ''}"
							>
								<div class="flex items-center gap-2">
									<div class="h-2 w-2 rounded-full bg-primary/40"></div>
									<span>{t.team?.name || 'Unknown Team'}</span>
								</div>
								<span class="text-[10px] uppercase font-bold text-muted-foreground bg-muted-foreground/10 px-1.5 py-0.5 rounded">
									{t.role}
								</span>
							</button>
						{/each}
					{/if}
				</div>
			</Card.Content>
		</Card.Root>

		<!-- Selected Team Detail -->
		<Card.Root class="lg:col-span-2">
			{#if $activeTeam?.team}
				<Card.Header>
					<div class="flex items-center justify-between">
						<div>
							<Card.Title>{$activeTeam?.team?.name || 'Selected Team'}</Card.Title>
							<Card.Description>Manage members and roles for this team</Card.Description>
						</div>
						<div class="flex items-center gap-2 px-3 py-1 bg-muted rounded-full">
							<Shield class="h-3.5 w-3.5 text-primary" />
							<span class="text-xs font-semibold">Your Role: {$activeTeam.role}</span>
						</div>
					</div>
				</Card.Header>
				<Card.Content class="space-y-6">
					<!-- Invite Section -->
					{#if $activeTeam?.role === 'Owner' || $activeTeam?.role === 'Admin'}
						<div class="grid gap-4 p-4 border rounded-lg bg-muted/30">
							<div class="flex items-center gap-2 text-sm font-medium">
								<UserPlus class="h-4 w-4" />
								Invite Member
							</div>
							<div class="flex flex-wrap gap-3">
								<div class="flex-1 min-w-[200px]">
									<Input placeholder="member@example.com" bind:value={inviteEmail} />
								</div>
								<div class="w-32">
									<select
										bind:value={inviteRole}
										class="w-full flex h-10 items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
									>
										{#each roles as r}
											<option value={r}>{r}</option>
										{/each}
									</select>
								</div>
								<Button onclick={inviteMember} disabled={inviting || !inviteEmail}>
									{inviting ? 'Inviting...' : 'Invite'}
								</Button>
							</div>
						</div>
					{/if}

					<!-- Members List -->
					<div class="space-y-4">
						<h4 class="font-medium text-sm flex items-center gap-2">
							Team Members
							<span class="text-xs font-normal text-muted-foreground bg-muted px-2 py-0.5 rounded-full">
								{selectedTeamMembers.length}
							</span>
						</h4>

						{#if loadingMembers}
							<div class="flex justify-center py-8">
								<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
							</div>
						{:else}
							<div class="grid gap-2">
								{#each selectedTeamMembers as member}
									<div class="flex items-center justify-between p-3 border rounded-lg bg-card/50">
										<div class="flex items-center gap-3">
											<div class="h-8 w-8 rounded-full bg-primary/10 flex items-center justify-center text-primary font-bold text-xs">
												{member.user_name?.substring(0, 2).toUpperCase() || member.user_email.substring(0, 2).toUpperCase()}
											</div>
											<div class="grid gap-0.5">
												<p class="text-sm font-medium leading-none flex items-center gap-1.5">
													{member.user_name || 'No Name'}
													{#if member.role === 'Owner'}
														<Star class="h-3 w-3 fill-yellow-400 text-yellow-400" />
													{/if}
												</p>
												<p class="text-xs text-muted-foreground">
													{member.user_email}
												</p>
											</div>
										</div>

										<div class="flex items-center gap-3">
											{#if ($activeTeam.role === 'Owner' || $activeTeam.role === 'Admin') && member.role !== 'Owner'}
												<select
													value={member.role}
													onchange={(e) => updateRole(member.user_id, e.currentTarget.value as TeamRole)}
													class="text-xs bg-transparent border-none focus:ring-0 cursor-pointer font-medium hover:text-primary transition-colors"
												>
													{#each roles as r}
														{#if r !== 'Owner' || $activeTeam.role === 'Owner'}
															<option value={r}>{r}</option>
														{/if}
													{/each}
												</select>

												<Button
													variant="ghost"
													size="icon"
													class="h-8 w-8 text-destructive hover:bg-destructive/10"
													onclick={() => removeMember(member.user_id)}
												>
													<Trash2 class="h-4 w-4" />
												</Button>
											{:else}
												<span class="text-xs font-medium text-muted-foreground px-2 py-1 bg-muted rounded">
													{member.role}
												</span>
											{/if}
										</div>
									</div>
								{/each}
							</div>
						{/if}
					</div>
				</Card.Content>
			{:else}
				<div class="flex flex-col items-center justify-center p-12 text-center h-[400px]">
					<div class="h-16 w-16 rounded-full bg-muted flex items-center justify-center mb-4">
						<Users class="h-8 w-8 text-muted-foreground" />
					</div>
					<h3 class="text-lg font-medium">No team selected</h3>
					<p class="text-sm text-muted-foreground max-w-sm mt-1">
						Please select a team from the list on the left to manage its members and settings.
					</p>
				</div>
			{/if}
		</Card.Root>
	</div>
</div>
