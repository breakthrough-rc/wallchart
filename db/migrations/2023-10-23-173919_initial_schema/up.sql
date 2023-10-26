create table if not exists worksites (
  id text primary key,
  name text not null
);

create table if not exists locations (
  id text primary key,
  name text not null,
  worksite_id text references worksites not null
);

create table if not exists shifts (
  id text primary key,
  name text not null,
  location_id text references locations not null
);

create table if not exists workers (
  id text primary key,
  first_name text not null,
  last_name text not null
);

create table if not exists shift_assignments (
  shift_id text references shifts not null,
  worker_id text references workers not null,
  primary key(shift_id, worker_id)
);

create table if not exists assessments (
  id text primary key,
  value integer not null,
  worker_id text references workers not null
);

create table if not exists activities (
  id text primary key,
  name text not null,
  icon text not null
);

create table if not exists tags (
  activity_id text references activities not null,
  worker_id text references workers not null,
  primary key(activity_id, worker_id)
);

