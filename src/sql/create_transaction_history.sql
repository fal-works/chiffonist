CREATE TABLE IF NOT EXISTS transaction_history (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  date TEXT NOT NULL,
  description TEXT NOT NULL DEFAULT '',
  amount INTEGER NOT NULL,
  major_category TEXT NOT NULL DEFAULT 'uncategorized',
  minor_category TEXT NOT NULL DEFAULT 'uncategorized',
  memo TEXT NOT NULL DEFAULT '',
  mf_transaction_id INTEGER UNIQUE -- FK
);