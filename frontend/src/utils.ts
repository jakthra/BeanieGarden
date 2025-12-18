import bcrypt from 'bcryptjs';

/**
 * Salt and hash a password using bcrypt
 * @param password - The plain text password to hash
 * @returns The salted and hashed password
 */
export async function saltAndHashPassword(password: string): Promise<string> {
  const saltRounds = 10;
  const hashedPassword = await bcrypt.hash(password, saltRounds);
  return hashedPassword;
}

/**
 * Verify a password against a hash
 * @param password - The plain text password to verify
 * @param hashedPassword - The hashed password to compare against
 * @returns True if the password matches, false otherwise
 */
export async function verifyPassword(
  password: string,
  hashedPassword: string
): Promise<boolean> {
  return await bcrypt.compare(password, hashedPassword);
}

/**
 * Dummy hash used for timing attack protection
 * Pre-generated bcrypt hash to use when user doesn't exist
 */
const DUMMY_HASH = '$2a$10$XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX';

/**
 * Verify password with timing attack protection
 * Always performs a hash comparison even if user doesn't exist
 * @param password - The plain text password to verify
 * @param hashedPassword - The hashed password to compare against (or null/undefined if user doesn't exist)
 * @returns True if the password matches, false otherwise
 */
export async function verifyPasswordSafe(
  password: string,
  hashedPassword: string | null | undefined
): Promise<boolean> {
  // Always perform a comparison, even if user doesn't exist
  // This prevents timing attacks that could reveal if a user exists
  const hashToCompare = hashedPassword || DUMMY_HASH;
  const isValid = await bcrypt.compare(password, hashToCompare);
  
  // Only return true if we had a real hash AND it matched
  return hashedPassword ? isValid : false;
}