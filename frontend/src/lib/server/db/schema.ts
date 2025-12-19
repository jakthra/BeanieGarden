import { pgTable, serial, integer, uuid, varchar, boolean, timestamp, index, pgEnum, numeric, unique } from 'drizzle-orm/pg-core';

export const userRole = pgEnum('role', ['admin', 'user'])

export const user = pgTable('user', {
  uuid: uuid('uuid').primaryKey().defaultRandom(),
  name: varchar('name').notNull(),
  email: varchar("email").notNull().unique(),
  active: boolean().default(true),
  created_at: timestamp().defaultNow(),
  pw_hash: varchar("pw_hash").notNull(),
  role: userRole(),
}, (table) => [
  index("user_uid_idx").on(table.uuid),
  index("user_email_idx").on(table.email)
]);


export const session = pgTable('session', {
  uuid: uuid('uuid').primaryKey().defaultRandom(),
  user_uid: uuid('user_uid')
    .notNull()
    .references(() => user.uuid, { onDelete: 'cascade' }),
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

export const common_plant = pgTable('common_plant', {
  id: serial('id').primaryKey(),
  common_danish_name: varchar('common_danish_name').notNull(),
  common_english_name: varchar('common_english_name').notNull(),
  da_wiki_url: varchar('da_wiki_url').notNull(),
  image_url: varchar('image_url').notNull(),
  description: varchar('description').notNull(),
  gbif_genus_key: integer('gbif_genus_key').notNull().references(() => gbif_genus.key, { onDelete: 'cascade' }).unique(),
}, (table) => [
  unique().on(table.common_danish_name, table.common_english_name)
]);

export const gbif_genus = pgTable('gbif_genus', {
  key: serial('key').primaryKey(),
  canonical_name: varchar('canonical_name').notNull(),
  scientific_name: varchar('scientific_name').notNull(),
  family: varchar('family').notNull(),
  genus: varchar('genus').notNull(),
  rank: varchar('rank').notNull(),
}, (table) => [
  index("gbif_genus_canonical_name_idx").on(table.canonical_name),
  index("gbif_genus_scientific_name_idx").on(table.scientific_name),
]);

export const growth = pgTable('growth', {
  uuid: uuid('uuid').primaryKey().defaultRandom(),
  growth_type: varchar('growth_type').notNull(),
  age_estimate: numeric('age_estimate').notNull(),
  height: numeric('height').notNull(),
  width: numeric('width').notNull(),
  common_plant_id: integer('common_plant_id').notNull().references(() => common_plant.id, { onDelete: 'cascade' }),
  created_by: uuid('created_by').notNull().references(() => user.uuid, { onDelete: 'cascade' }),
  created_at: timestamp('created_at', { withTimezone: true, mode: 'date' }).notNull(),
  active: boolean().default(true),
}, (table) => [
  index("growth_uid_idx").on(table.uuid),
  index("growth_common_plant_id_idx").on(table.common_plant_id),
]);

export const gardening_task = pgTable('gardening_task', {
  uuid: uuid('uuid').primaryKey().defaultRandom(),
  title: varchar('title').notNull(),
  priority: varchar('priority').notNull(),
  time_required: numeric('time_required').notNull(),
  description: varchar('description').notNull(),
  tips: varchar('tips').notNull(),
  created_by: uuid('created_by').notNull().references(() => user.uuid, { onDelete: 'cascade' }),
}, (table) => [
  index("gardening_task_uid_idx").on(table.uuid),
]);

export const gardening_task_growth_assoication = pgTable('gardening_task_growth_assoication', {
   uuid: uuid('uuid').primaryKey().defaultRandom(),
   gardening_task_uid: uuid('gardening_task_uid').notNull().references(() => gardening_task.uuid, { onDelete: 'cascade' }),
   growth_uuid: uuid('growth_uuid').notNull().references(() => growth.uuid, { onDelete: 'cascade' }),
 }, (table) => [
   index("gardening_task_growth_assoication_uid_idx").on(table.uuid),
 ]);

