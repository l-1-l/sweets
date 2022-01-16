create table users (
  id bytea primary key not null,
  name text not null,
  bio text,
  birthdate timestamptz not null,
  gender text not null,
  avatar text,
  -- 预留字段 是否为未成年人
  is_minor boolean,
  status text,
  custom_status_text text,
  custom_stauts_emoji text,
  custom_status_expire_at timestamptz,
  ext text,
  created_at timestamptz not null default now(),
  updated_at timestamptz
);