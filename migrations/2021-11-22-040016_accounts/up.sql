-- Your SQL goes here
create table accounts (
  id bytea primary key not null,
  user_id bytea,
  name text,
  -- 86-13888888888
  mobile text not null,
  mobile_prefix text not null,
  email text,
  hash text,
  status smallint not null,
  ext text,
  created_at timestamptz not null default now(),
  updated_at timestamptz
);
-- YeLPGMqLHkfdQ_uX
-- 2022-01-15 13:41:44.963881+00