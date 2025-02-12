CREATE TEMPORARY TABLE categorized_amazon_ohfd (
  id INTEGER NOT NULL UNIQUE, -- REFERENCES amazon_ohfd(id)
  channel TEXT NOT NULL,
  category TEXT NOT NULL,
  sub_category TEXT NOT NULL
);