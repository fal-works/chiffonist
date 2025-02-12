CREATE TABLE IF NOT EXISTS amazon_ohfd_manual_categorization (
  order_no INTEGER NOT NULL,
  product_name TEXT NOT NULL,
  category TEXT NOT NULL,
  sub_category TEXT,
  PRIMARY KEY (order_no, product_name)
);