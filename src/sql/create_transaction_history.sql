CREATE TABLE IF NOT EXISTS transaction_history (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  date TEXT NOT NULL,
  description TEXT NOT NULL DEFAULT '',
  amount INTEGER NOT NULL,
  category TEXT NOT NULL DEFAULT 'none',
  sub_category TEXT NOT NULL DEFAULT 'none',
  memo TEXT NOT NULL DEFAULT '',
  mf_transaction_id INTEGER UNIQUE -- FK
);