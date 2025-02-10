CREATE TEMPORARY TABLE categorized_mf_transaction (
  id INTEGER NOT NULL UNIQUE,
  channel TEXT NOT NULL,
  category TEXT NOT NULL,
  sub_category TEXT NOT NULL
);