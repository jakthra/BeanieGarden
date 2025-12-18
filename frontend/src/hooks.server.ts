import { getUserFromSession } from '$lib/server/db/queries';
import { redirect } from '@sveltejs/kit';

// src/hooks.server.js
export async function handle({ event, resolve }) {
    const sessionUid = event.cookies.get('sessionuid');

    const protectedPaths = ["/app"]
    const path = event.url.pathname

    if (!sessionUid && protectedPaths.some((protectedPath) => path.includes(protectedPath))) {
        throw redirect(301, '/login')
    }

    if (sessionUid) {
        const user = await getUserFromSession(sessionUid);

        if (!user) {
            throw redirect(301, '/login')
        }

        event.locals.user = user;

    }

    return resolve(event);
}