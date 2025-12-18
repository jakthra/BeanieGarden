import { drizzle } from 'drizzle-orm/postgres-js';
import postgres from 'postgres';
import * as schema from '../src/lib/server/db/schema.js';
import { saltAndHashPassword } from '../src/utils.js';
import { randomUUID } from 'crypto';
import * as dotenv from 'dotenv';
import { fileURLToPath } from 'url';
import { dirname, resolve } from 'path';

// Load environment variables from .env file
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
dotenv.config({ path: resolve(__dirname, '../.env') });

const DATABASE_URL = process.env.DATABASE_URL;

if (!DATABASE_URL) {
  console.error('ERROR: DATABASE_URL is not set in .env file');
  process.exit(1);
}

async function addAdmin() {
  const email = process.argv[2];
  const password = process.argv[3];

  if (!email || !password) {
    console.error('Usage: npm run db:add-admin <email> <password>');
    console.error('Example: npm run db:add-admin admin@example.com MySecurePassword123');
    process.exit(1);
  }

  // Validate email format
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  if (!emailRegex.test(email)) {
    console.error('ERROR: Invalid email format');
    process.exit(1);
  }

  // Validate password strength
  if (password.length < 8) {
    console.error('ERROR: Password must be at least 8 characters long');
    process.exit(1);
  }

  try {
    const client = postgres(DATABASE_URL);
    const db = drizzle(client, { schema });

    // Check if user already exists
    const existingUser = await db.query.user.findFirst({
      where: (user, { eq }) => eq(user.email, email)
    });

    if (existingUser) {
      console.error(`ERROR: User with email ${email} already exists`);
      await client.end();
      process.exit(1);
    }

    // Hash the password
    const pwHash = await saltAndHashPassword(password);

    // Insert the admin user
    const uid = randomUUID();
    await db.insert(schema.user).values({
      uid,
      email,
      pw_hash: pwHash,
      active: true,
      role: "admin",
      name: "Jakob Thrane"
    });

    console.log(`✓ Admin user created successfully`);
    console.log(`  Email: ${email}`);
    console.log(`  UID: ${uid}`);

    await client.end();
    process.exit(0);
  } catch (error) {
    console.error('ERROR: Failed to create admin user');
    console.error(error);
    process.exit(1);
  }
}

addAdmin();
