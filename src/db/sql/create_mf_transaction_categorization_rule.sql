CREATE TABLE IF NOT EXISTS mf_transaction_categorization_rule (
  id INTEGER NOT NULL PRIMARY KEY,
  mf_include_flag INTEGER,
  mf_occurrence_date_min TEXT,
  mf_occurrence_date_max TEXT,
  mf_particulars_glob TEXT,
  mf_amount_min INTEGER,
  mf_amount_max INTEGER,
  mf_financial_institution TEXT,
  mf_major_category TEXT,
  mf_intermediate_category TEXT,
  mf_memo_glob TEXT,
  mf_transfer_flag INTEGER,
  new_category TEXT NOT NULL,
  new_sub_category TEXT
);
CREATE INDEX IF NOT EXISTS idx_mf_transaction_categorization_rule_occurrence_date ON mf_transaction_categorization_rule (
  mf_occurrence_date_min,
  mf_occurrence_date_max
);
CREATE INDEX IF NOT EXISTS idx_mf_transaction_categorization_rule_amount ON mf_transaction_categorization_rule (mf_amount_min, mf_amount_max);
CREATE INDEX IF NOT EXISTS idx_mf_transaction_categorization_rule_financial_institution ON mf_transaction_categorization_rule (mf_financial_institution);
CREATE INDEX IF NOT EXISTS idx_mf_transaction_categorization_rule_categories ON mf_transaction_categorization_rule (
  mf_major_category,
  mf_intermediate_category
);