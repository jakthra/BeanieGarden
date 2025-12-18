import { findUserByEmail, createSession } from '$lib/server/db/queries';
import { fail, redirect } from '@sveltejs/kit';
import { saltAndHashPassword } from '../../utils';
import type { Actions } from './$types';
import bcrypt from 'bcryptjs';

export const actions = {
    default: async ({ cookies, request, getClientAddress }) => {

        const data = await request.formData();

        const email = data.get('email');
        const password = data.get('password');
        const ip = getClientAddress()
        const user_agent: string | undefined = request.headers.get('user-agent') ?? undefined

        if (!email || !password) {
            return fail(400, { error: 'Email and password are required' });
        }

        const user = await findUserByEmail(email as string);
        if (!user) {
            return fail(401, { error: 'Invalid credentials' });
        }

        if (!user.active) {
            return fail(401, { error: 'User deactivated' });
        }

        if (!await bcrypt.compare(password as string, user.pw_hash as string)) {
            return fail(401, { error: 'Invalid credentials' });
        }
        const session = await createSession({ user_uid: user.uid, ip, user_agent });
        cookies.set('sessionuid', session.uid, { path: '/', expires: session.expires_at });

        throw redirect(303, '/');

    },
} satisfies Actions;