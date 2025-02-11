CREATE TABLE IF NOT EXISTS mf_transaction_manual_categorization (
  mf_original_id TEXT NOT NULL PRIMARY KEY,
  category TEXT NOT NULL,
  sub_category TEXT NOT NULL
);