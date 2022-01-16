create table stories (
  id bytea primary key not null,
  author_id bytea not null,
  content text,
  location point,
  is_public boolean not null default true,
  invisible boolean not null default false,
  weight bigint,
  created_at timestamptz not null,
  updated_at timestamptz
);