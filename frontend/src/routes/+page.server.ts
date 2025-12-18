// routes/+page.server.js
import { redirect } from '@sveltejs/kit';

export function load({ locals }) {

  if (!locals.user) {
    throw redirect(302, '/login');
  }

  throw redirect(302, "/app/tasks")
}
