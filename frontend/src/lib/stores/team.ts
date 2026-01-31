import { writable } from "svelte/store";
import { browser } from "$app/environment";
import type { TeamResponse } from "$lib/api";

function createTeamStore() {
  let initialValue: TeamResponse | null = null;
  if (browser) {
    const stored = localStorage.getItem("activeTeam");
    if (stored) {
      try {
        const parsed = JSON.parse(stored);
        if (parsed && parsed.team && parsed.team.id && parsed.role) {
          initialValue = parsed;
        } else {
          localStorage.removeItem("activeTeam");
        }
      } catch (e) {
        localStorage.removeItem("activeTeam");
      }
    }
  }

  const { subscribe, set, update } = writable<TeamResponse | null>(
    initialValue,
  );

  return {
    subscribe,
    setActiveTeam: (team: TeamResponse | null) => {
      if (browser) {
        if (team) {
          localStorage.setItem("activeTeam", JSON.stringify(team));
        } else {
          localStorage.removeItem("activeTeam");
        }
      }
      set(team);
    },
    reset: () => {
      if (browser) {
        localStorage.removeItem("activeTeam");
      }
      set(null);
    },
  };
}

export const activeTeam = createTeamStore();
