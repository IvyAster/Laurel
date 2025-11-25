CREATE
DATABASE laurel_system
  WITH
  OWNER = admin
  ENCODING = 'UTF8'
  LC_COLLATE = 'en_US.UTF-8'
  LC_CTYPE = 'en_US.UTF-8'
  TEMPLATE = template0;

CREATE TABLE account
(
    id            BIGSERIAL   NOT NULL PRIMARY KEY,
    account_id    VARCHAR(40) NOT NULL,
    account_name  VARCHAR(64) NOT NULL,
    account_state VARCHAR(64) NOT NULL,
    account_type  VARCHAR(20) NOT NULL,
    cts           TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    uts           TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT uniq_ai UNIQUE (account_id),
    CONSTRAINT uniq_aa UNIQUE (account_name, account_type)
);

COMMENT
ON TABLE account IS '账户表';
COMMENT
ON COLUMN account.id IS '自增id';
COMMENT
ON COLUMN account.account_id IS '子账户id';
COMMENT
ON COLUMN account.account_name IS '账户名称';
COMMENT
ON COLUMN account.account_state IS '账户状态';
COMMENT
ON COLUMN account.account_type IS '账户类型';
COMMENT
ON COLUMN account.cts IS '创建时间';
COMMENT
ON COLUMN account.uts IS '更新时间';

CREATE TABLE passport
(
    id         BIGSERIAL   NOT NULL PRIMARY KEY,
    account_id VARCHAR(40) NOT NULL,
    salt       VARCHAR(64) NOT NULL,
    password   VARCHAR(64) NOT NULL,
    cts        TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    uts        TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT uniq_ai UNIQUE (account_id),
);

COMMENT
ON TABLE passport IS '账户密码户表';
COMMENT
ON COLUMN passport.id IS '自增id';
COMMENT
ON COLUMN passport.account_id IS '账户id';
COMMENT
ON COLUMN passport.salt IS '密码盐';
COMMENT
ON COLUMN passport.password IS '账户密码';
COMMENT
ON COLUMN passport.cts IS '创建时间';
COMMENT
ON COLUMN passport.uts IS '更新时间';

CREATE TABLE "profile"
(
    "id"            BIGSERIAL PRIMARY KEY,
    "account_id"    VARCHAR(40) NOT NULL,
    "profile_key"   VARCHAR(40) NOT NULL,
    "profile_value" TEXT                 DEFAULT NULL,
    "cts"           TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "uts"           TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX "profile_ai_pk" ON "profile" ("account_id", "profile_key");

COMMENT
ON TABLE "profile" IS '名片资料表';
COMMENT
ON COLUMN "profile"."id" IS '自增id';
COMMENT
ON COLUMN "profile"."account_id" IS '账户';
COMMENT
ON COLUMN "profile"."profile_key" IS '名片资料存储key';
COMMENT
ON COLUMN "profile"."profile_value" IS '名片资料存储value';
COMMENT
ON COLUMN "profile"."cts" IS '创建时间';
COMMENT
ON COLUMN "profile"."uts" IS '更新时间';
