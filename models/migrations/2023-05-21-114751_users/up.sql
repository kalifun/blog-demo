-- Your SQL goes here
-- Table Definition
CREATE TABLE "public"."users" (
    "id" uuid NOT NULL,
    "name" varchar NOT NULL,
    "create_time" timestamp NOT NULL,
    "update_time" timestamp NOT NULL,
    PRIMARY KEY ("id")
);

-- Column Comment
COMMENT ON COLUMN "public"."users"."name" IS '用户名';
COMMENT ON COLUMN "public"."users"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."users"."update_time" IS '更新时间';
