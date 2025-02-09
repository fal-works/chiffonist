CREATE TEMPORARY TABLE categorized_mf_transaction (
  id INTEGER UNIQUE,
  category TEXT NOT NULL,
  sub_category TEXT NOT NULL
);