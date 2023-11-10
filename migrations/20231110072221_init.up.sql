CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS games {
     "id" uuid PRIMARY KEY NOT DEFAULT (uuid_generate_v4());
    "field_name_" varchar NOT ,
    "address" varchar NOT NULL,
    "day" varchar NOT NULL,
     "created_at" TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
     "updated_at" TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
}