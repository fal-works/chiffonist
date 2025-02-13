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
CREATE INDEX IF NOT EXISTS idx_mf_occurrence_date ON mf_transaction (occurrence_date);
CREATE INDEX IF NOT EXISTS idx_mf_financial_institution ON mf_transaction (financial_institution);
CREATE INDEX IF NOT EXISTS idx_mf_categories ON mf_transaction (major_category, intermediate_category);