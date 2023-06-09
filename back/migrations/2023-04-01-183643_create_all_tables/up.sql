-- Your SQL goes here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "user" (
  userid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  username VARCHAR UNIQUE NOT NULL,
  pwd VARCHAR NOT NULL,
  isnotionoauth BOOLEAN NOT NULL DEFAULT FALSE,
  lastlogin TIMESTAMP,
  datecreated TIMESTAMP NOT NULL DEFAULT now(),
  token VARCHAR
);

CREATE TABLE template ( 
    templateid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    userid UUID ,
    templatename varchar NOT NULL DEFAULT '',
    creationdate timestamp NOT NULL default now(),
    weekdays BOOLEAN[] NOT NULL DEFAULT '{false, false, false, false, false, false, false}',
    updatedate timestamp,
    CONSTRAINT fk_userid_template
      FOREIGN KEY(userid) 
      REFERENCES public.user(userid)
);

CREATE TABLE templating_task (
    temptaskid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    userid UUID,
    templateid UUID DEFAULT null,
    "content" VARCHAR, 
    creationdate TIMESTAMP NOT NULL DEFAULT now(),
    duetime TIMESTAMP,
    CONSTRAINT fk_userid_task
      FOREIGN KEY(userid) 
	  REFERENCES public.user(userid),
    CONSTRAINT fk_templateid_task
      FOREIGN KEY(templateid) 
	  REFERENCES public."template"(templateid)
);

CREATE TABLE executable_task (
    exetaskid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    userid UUID,
    templateid UUID DEFAULT null,
    "content" VARCHAR, 
    checked BOOL NOT NULL DEFAULT false,
    creationdate TIMESTAMP NOT NULL DEFAULT now(),
    duetime TIMESTAMP,
    CONSTRAINT fk_userid_task
      FOREIGN KEY(userid) 
	  REFERENCES public.user(userid),
    CONSTRAINT fk_templateid_task
      FOREIGN KEY(templateid) 
	  REFERENCES public."template"(templateid)
);
