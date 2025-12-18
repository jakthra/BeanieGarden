export async function load({ locals }) {
  return {
    user: locals.user,
  };
}

import { revokeSession } from '$lib/server/db/queries';
import { redirect } from '@sveltejs/kit';
import type { Actions } from './$types';

export const actions = {
  deleteCookie: async ({ cookies, locals }) => {
    const session_uid = cookies.get("sessionuid")
    if (session_uid) {
      await revokeSession(session_uid)
      cookies.delete('sessionuid', { path: '/' });
      locals.user = undefined
    }
    throw redirect(308, "/login")
  }
} satisfies Actions;
