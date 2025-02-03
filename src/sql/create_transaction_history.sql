CREATE TABLE IF NOT EXISTS transaction_history (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  date TEXT NOT NULL,
  description TEXT NOT NULL,
  amount INTEGER NOT NULL,
  major_category TEXT NOT NULL,
  minor_category TEXT NOT NULL,
  memo TEXT,
  mf_transaction_id INTEGER UNIQUE -- FK
);