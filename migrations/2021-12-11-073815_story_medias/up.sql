create table story_medias (
  id serial primary key,
  author_id bytea not null,
  story_id bytea not null references stories(id) on delete cascade,
  kind text not null,
  url text not null,
  cover text,
  weight smallint,
  ratio_x smallint,
  ratio_y smallint,
  ext text,
  created_at timestamptz not null default now(),
  updated_at timestamptz
);