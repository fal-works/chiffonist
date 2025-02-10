CREATE TABLE IF NOT EXISTS mf_transaction (
  id INTEGER NOT NULL PRIMARY KEY,
  include_flag INTEGER NOT NULL,
  occurrence_date TEXT NOT NULL,
  particulars TEXT NOT NULL,
  amount INTEGER NOT NULL,
  financial_institution TEXT NOT NULL,
  major_category TEXT NOT NULL,
  intermediate_category TEXT NOT NULL,
  memo TEXT NOT NULL,
  transfer_flag INTEGER NOT NULL,
  mf_original_id TEXT NOT NULL UNIQUE
);