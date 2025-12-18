import * as schema from './schema'
import { db } from '.'
import { and, eq, gt } from 'drizzle-orm'

export async function findUserByEmail(email: string) {
  return db.query.user.findFirst({
    where: eq(schema.user.email, email),
  })
}

export interface createSessionSchema {
  user_uid: string
  ip?: string
  user_agent?: string
}

export async function createSession({ user_uid, ip, user_agent }: createSessionSchema) {
  const expiresAt = new Date();
  expiresAt.setDate(expiresAt.getDate() + 14);
  const [session] = await db.insert(schema.session).values({ user_uid: user_uid, expires_at: expiresAt, ip_address: ip, user_agent: user_agent }).returning()
  return session
}

export async function getUserFromSession(session_uid: string) {
  const session = await db.query.session.findFirst({
    where: and(
      eq(schema.session.uid, session_uid),
      gt(schema.session.expires_at, new Date())
    )
  })

  if (!session) return null;

  return db.query.user.findFirst({
    where: eq(schema.user.uid, session.user_uid)
  })
}

export async function revokeSession(session_uid: string) {
  await db.update(schema.session).set({ expires_at: new Date() }).where(eq(schema.session.uid, session_uid))
}
