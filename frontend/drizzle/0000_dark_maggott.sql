CREATE TABLE "session" (
	"uid" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"user_uid" uuid NOT NULL,
	"expires_at" timestamp with time zone NOT NULL,
	"created_at" timestamp with time zone DEFAULT now() NOT NULL,
	"ip_address" varchar(45),
	"user_agent" varchar,
	"last_activity_at" timestamp with time zone DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE "user" (
	"uid" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"email" varchar NOT NULL,
	"active" boolean DEFAULT true,
	"created_at" timestamp DEFAULT now(),
	"pw_hash" varchar,
	CONSTRAINT "user_email_unique" UNIQUE("email")
);
--> statement-breakpoint
ALTER TABLE "session" ADD CONSTRAINT "session_user_uid_user_uid_fk" FOREIGN KEY ("user_uid") REFERENCES "public"."user"("uid") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
CREATE INDEX "session_user_uid_idx" ON "session" USING btree ("user_uid");--> statement-breakpoint
CREATE INDEX "user_uid_idx" ON "user" USING btree ("uid");--> statement-breakpoint
CREATE INDEX "user_email_idx" ON "user" USING btree ("email");