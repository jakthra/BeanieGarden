// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
import type { user } from '$lib/server/db/schema'; // adjust path to your schema
import type { InferSelectModel } from 'drizzle-orm';
type User = InferSelectModel<typeof user>;
declare global {
  namespace App {

    // interface Error {}
    interface Locals {
      user?: User
    }
    // interface PageData {}
    // interface PageState {}
    // interface Platform {}
  }
}

export { };

