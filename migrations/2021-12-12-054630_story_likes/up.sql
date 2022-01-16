create table story_likes (
  id bigint not null,
  user_id bytea not null references users (id) on delete cascade,
  story_id bytea not null references stories (id) on delete cascade,
  created_at timestamptz not null default now()
)