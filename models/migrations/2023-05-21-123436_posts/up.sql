-- Your SQL goes here
-- Table Definition
CREATE TABLE "public"."posts" (
    "id" uuid NOT NULL,
    "title" varchar NOT NULL,
    "desc" text,
    "body" text NOT NULL,
    "user_id" uuid NOT NULL,
    "tag_id" uuid,
    "state" int2 NOT NULL,
    "create_time" timestamp NOT NULL,
    "update_time" timestamp NOT NULL,
    CONSTRAINT "posts_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "public"."users"("id") ON DELETE SET NULL,
    CONSTRAINT "posts_tag_id_fkey" FOREIGN KEY ("tag_id") REFERENCES "public"."tags"("id") ON DELETE SET NULL,
    PRIMARY KEY ("id")
);

-- Column Comment
COMMENT ON COLUMN "public"."posts"."title" IS '标题';
COMMENT ON COLUMN "public"."posts"."desc" IS '简述';
COMMENT ON COLUMN "public"."posts"."body" IS '内容';
COMMENT ON COLUMN "public"."posts"."user_id" IS '用户id';
COMMENT ON COLUMN "public"."posts"."tag_id" IS '标签id';
COMMENT ON COLUMN "public"."posts"."state" IS '状态';
COMMENT ON COLUMN "public"."posts"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."posts"."update_time" IS '更新时间';
