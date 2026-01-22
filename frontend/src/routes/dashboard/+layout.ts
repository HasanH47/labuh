import { redirect } from "@sveltejs/kit";
import type { LayoutLoad } from "./$types";
import { browser } from "$app/environment";
import { api } from "$lib/api";

export const load: LayoutLoad = async () => {
  // Only check auth on browser (client-side)
  if (browser) {
    const token = localStorage.getItem("token");

    if (!token) {
      // Redirect to login if no token
      throw redirect(302, "/login");
    }

    // Verify token is still valid by calling /api/auth/me
    const result = await api.auth.me();

    if (result.error) {
      // Token is invalid or expired, clear it and redirect
      localStorage.removeItem("token");
      localStorage.removeItem("user");
      throw redirect(302, "/login");
    }

    // Token is valid, return the user data
    return { user: result.data };
  }

  return {};
};
