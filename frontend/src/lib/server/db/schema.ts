import { pgTable, serial, integer, uuid, varchar, boolean, timestamp, index, pgEnum } from 'drizzle-orm/pg-core';

export const userRole = pgEnum('role', ['admin', 'user'])

export const user = pgTable('user', {
	uid: uuid('uid').primaryKey().defaultRandom(),
	name: varchar('name').notNull(),
	email: varchar("email").notNull().unique(),
	active: boolean().default(true),
	created_at: timestamp().defaultNow(),
	pw_hash: varchar("pw_hash").notNull(),
	role: userRole(),
}, (table) => [
	index("user_uid_idx").on(table.uid),
	index("user_email_idx").on(table.email)
]);


export const session = pgTable('session', {
	uid: uuid('uid').primaryKey().defaultRandom(),
	user_uid: uuid('user_uid')
		.notNull()
		.references(() => user.uid, { onDelete: 'cascade' }),
	expires_at: timestamp('expires_at', { withTimezone: true, mode: 'date' })
		.notNull(),
	created_at: timestamp('created_at', { withTimezone: true, mode: 'date' })
		.notNull()
		.defaultNow(),
	ip_address: varchar('ip_address', { length: 45 }), // IPv6 max length
	user_agent: varchar('user_agent'),
	last_activity_at: timestamp('last_activity_at', { withTimezone: true, mode: 'date' })
		.notNull()
		.defaultNow(),
}, (table) => [
	index("session_user_uid_idx").on(table.user_uid),
]);