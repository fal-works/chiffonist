CREATE TEMPORARY TABLE IF NOT EXISTS amazon_ohfd_addition (
  order_date TEXT NOT NULL,
  order_no TEXT NOT NULL,
  product_name TEXT NOT NULL,
  product_info TEXT NOT NULL,
  amount INTEGER NOT NULL,
  credit_card TEXT
);