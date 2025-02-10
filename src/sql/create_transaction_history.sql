CREATE TABLE IF NOT EXISTS transaction_history (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  occurrence_date TEXT NOT NULL,
  particulars TEXT NOT NULL DEFAULT '',
  amount INTEGER NOT NULL,
  channel TEXT NOT NULL,
  category TEXT NOT NULL DEFAULT 'none',
  sub_category TEXT NOT NULL DEFAULT 'none',
  memo TEXT NOT NULL DEFAULT '',
  mf_transaction_id INTEGER UNIQUE -- FK
);