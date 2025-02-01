CREATE TABLE IF NOT EXISTS mf_transaction (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  include INTEGER NOT NULL,
  date TEXT NOT NULL,
  description TEXT NOT NULL,
  amount INTEGER NOT NULL,
  financial_institution TEXT NOT NULL,
  major_category TEXT NOT NULL,
  minor_category TEXT NOT NULL,
  memo TEXT,
  transfer INTEGER NOT NULL,
  mf_original_id TEXT NOT NULL UNIQUE
);