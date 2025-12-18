CREATE TYPE "public"."role" AS ENUM('admin', 'user');--> statement-breakpoint
ALTER TABLE "user" ALTER COLUMN "pw_hash" SET NOT NULL;--> statement-breakpoint
ALTER TABLE "user" ADD COLUMN "name" varchar NOT NULL;--> statement-breakpoint
ALTER TABLE "user" ADD COLUMN "role" "role";