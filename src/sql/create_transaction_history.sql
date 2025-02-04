CREATE TABLE IF NOT EXISTS transaction_history (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  date TEXT NOT NULL,
  description TEXT DEFAULT '',
  amount INTEGER NOT NULL,
  major_category TEXT DEFAULT 'uncategorized',
  minor_category TEXT DEFAULT 'uncategorized',
  memo TEXT DEFAULT '',
  mf_transaction_id INTEGER UNIQUE -- FK
);