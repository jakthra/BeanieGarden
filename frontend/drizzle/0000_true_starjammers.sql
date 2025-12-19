CREATE TYPE "public"."role" AS ENUM('admin', 'user');--> statement-breakpoint
CREATE TABLE "common_plant" (
	"id" serial PRIMARY KEY NOT NULL,
	"common_danish_name" varchar NOT NULL,
	"common_english_name" varchar NOT NULL,
	"da_wiki_url" varchar NOT NULL,
	"image_url" varchar NOT NULL,
	"description" varchar NOT NULL,
	"gbif_genus_key" integer NOT NULL,
	CONSTRAINT "common_plant_common_danish_name_common_english_name_unique" UNIQUE("common_danish_name","common_english_name")
);
--> statement-breakpoint
CREATE TABLE "gardening_task" (
	"uuid" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"title" varchar NOT NULL,
	"priority" varchar NOT NULL,
	"time_required" numeric NOT NULL,
	"description" varchar NOT NULL,
	"tips" varchar NOT NULL,
	"created_by" uuid NOT NULL
);
--> statement-breakpoint
CREATE TABLE "gardening_task_growth_assoication" (
	"uuid" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"gardening_task_uid" uuid NOT NULL,
	"growth_uuid" uuid NOT NULL
);
--> statement-breakpoint
CREATE TABLE "gbif_genus" (
	"key" serial PRIMARY KEY NOT NULL,
	"canonical_name" varchar NOT NULL,
	"scientific_name" varchar NOT NULL,
	"family" varchar NOT NULL,
	"genus" varchar NOT NULL,
	"rank" varchar NOT NULL
);
--> statement-breakpoint
CREATE TABLE "growth" (
	"uuid" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"growth_type" varchar NOT NULL,
	"age_estimate" numeric NOT NULL,
	"height" numeric NOT NULL,
	"width" numeric NOT NULL,
	"common_plant_id" integer NOT NULL,
	"created_by" uuid NOT NULL,
	"created_at" timestamp with time zone NOT NULL,
	"active" boolean DEFAULT true
);
--> statement-breakpoint
CREATE TABLE "session" (
	"uuid" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"user_uid" uuid NOT NULL,
	"expires_at" timestamp with time zone NOT NULL,
	"created_at" timestamp with time zone DEFAULT now() NOT NULL,
	"ip_address" varchar(45),
	"user_agent" varchar,
	"last_activity_at" timestamp with time zone DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE "user" (
	"uuid" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"name" varchar NOT NULL,
	"email" varchar NOT NULL,
	"active" boolean DEFAULT true,
	"created_at" timestamp DEFAULT now(),
	"pw_hash" varchar NOT NULL,
	"role" "role",
	CONSTRAINT "user_email_unique" UNIQUE("email")
);
--> statement-breakpoint
ALTER TABLE "common_plant" ADD CONSTRAINT "common_plant_gbif_genus_key_gbif_genus_key_fk" FOREIGN KEY ("gbif_genus_key") REFERENCES "public"."gbif_genus"("key") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "gardening_task" ADD CONSTRAINT "gardening_task_created_by_user_uuid_fk" FOREIGN KEY ("created_by") REFERENCES "public"."user"("uuid") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "gardening_task_growth_assoication" ADD CONSTRAINT "gardening_task_growth_assoication_gardening_task_uid_gardening_task_uuid_fk" FOREIGN KEY ("gardening_task_uid") REFERENCES "public"."gardening_task"("uuid") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "gardening_task_growth_assoication" ADD CONSTRAINT "gardening_task_growth_assoication_growth_uuid_growth_uuid_fk" FOREIGN KEY ("growth_uuid") REFERENCES "public"."growth"("uuid") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "growth" ADD CONSTRAINT "growth_common_plant_id_common_plant_id_fk" FOREIGN KEY ("common_plant_id") REFERENCES "public"."common_plant"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "growth" ADD CONSTRAINT "growth_created_by_user_uuid_fk" FOREIGN KEY ("created_by") REFERENCES "public"."user"("uuid") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "session" ADD CONSTRAINT "session_user_uid_user_uuid_fk" FOREIGN KEY ("user_uid") REFERENCES "public"."user"("uuid") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
CREATE INDEX "common_plant_common_danish_name_idx" ON "common_plant" USING btree ("common_danish_name");--> statement-breakpoint
CREATE INDEX "common_plant_common_english_name_idx" ON "common_plant" USING btree ("common_english_name");--> statement-breakpoint
CREATE INDEX "gardening_task_uid_idx" ON "gardening_task" USING btree ("uuid");--> statement-breakpoint
CREATE INDEX "gardening_task_growth_assoication_uid_idx" ON "gardening_task_growth_assoication" USING btree ("uuid");--> statement-breakpoint
CREATE INDEX "gbif_genus_canonical_name_idx" ON "gbif_genus" USING btree ("canonical_name");--> statement-breakpoint
CREATE INDEX "gbif_genus_scientific_name_idx" ON "gbif_genus" USING btree ("scientific_name");--> statement-breakpoint
CREATE INDEX "growth_uid_idx" ON "growth" USING btree ("uuid");--> statement-breakpoint
CREATE INDEX "growth_common_plant_id_idx" ON "growth" USING btree ("common_plant_id");--> statement-breakpoint
CREATE INDEX "session_user_uid_idx" ON "session" USING btree ("user_uid");--> statement-breakpoint
CREATE INDEX "user_uid_idx" ON "user" USING btree ("uuid");--> statement-breakpoint
CREATE INDEX "user_email_idx" ON "user" USING btree ("email");