CREATE TABLE IF NOT EXISTS amazon_ohfd_categorization_rule (
  id INTEGER NOT NULL PRIMARY KEY,
  order_date_min TEXT,
  order_date_max TEXT,
  product_name_glob TEXT,
  product_info_glob TEXT,
  amount_min INTEGER,
  amount_max INTEGER,
  credit_card TEXT,
  new_category TEXT NOT NULL,
  new_sub_category TEXT
);
CREATE INDEX IF NOT EXISTS idx_amazon_ohfd_categorization_rule_order_date ON amazon_ohfd_categorization_rule (
  order_date_min,
  order_date_max
);
CREATE INDEX IF NOT EXISTS idx_amazon_ohfd_categorization_rule_amount ON amazon_ohfd_categorization_rule (amount_min, amount_max);
CREATE INDEX IF NOT EXISTS idx_amazon_ohfd_categorization_rule_credit_card ON amazon_ohfd_categorization_rule (credit_card);