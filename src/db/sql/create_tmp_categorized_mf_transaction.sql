CREATE TEMPORARY TABLE categorized_mf_transaction (
  id INTEGER UNIQUE,
  channel TEXT NOT NULL,
  category TEXT NOT NULL,
  sub_category TEXT NOT NULL
);