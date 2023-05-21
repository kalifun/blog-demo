-- Your SQL goes here
-- Table Definition
CREATE TABLE "public"."tags" (
    "id" uuid NOT NULL,
    "name" varchar NOT NULL,
    "create_time" timestamp NOT NULL,
    "update_time" timestamp NOT NULL,
    PRIMARY KEY ("id")
);

-- Column Comment
COMMENT ON COLUMN "public"."tags"."name" IS '标签名';
COMMENT ON COLUMN "public"."tags"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."tags"."update_time" IS '更新时间';
