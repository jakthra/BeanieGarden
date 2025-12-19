ALTER TABLE "session" RENAME COLUMN "user_uid" TO "user_uuid";--> statement-breakpoint
ALTER TABLE "session" DROP CONSTRAINT "session_user_uid_user_uuid_fk";
--> statement-breakpoint
DROP INDEX "session_user_uid_idx";--> statement-breakpoint
DROP INDEX "user_uid_idx";--> statement-breakpoint
ALTER TABLE "session" ADD CONSTRAINT "session_user_uuid_user_uuid_fk" FOREIGN KEY ("user_uuid") REFERENCES "public"."user"("uuid") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
CREATE INDEX "session_user_uuid_idx" ON "session" USING btree ("user_uuid");--> statement-breakpoint
CREATE INDEX "user_uuid_idx" ON "user" USING btree ("uuid");