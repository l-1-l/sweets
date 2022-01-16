create table story_comments (
  id serial primary key,
  user_id bytea not null references users(id) on delete cascade,
  story_id bytea not null references stories(id) on delete cascade,
  parent_id bytea,
  comment text not null,
  created_at timestamptz not null default now(),
  updated_at timestamptz
);