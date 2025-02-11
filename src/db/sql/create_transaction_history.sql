CREATE TABLE IF NOT EXISTS transaction_history (
  id INTEGER NOT NULL PRIMARY KEY,
  occurrence_year INTEGER GENERATED ALWAYS AS (CAST(SUBSTR(occurrence_date, 1, 4) AS INTEGER)) STORED,
  occurrence_month INTEGER GENERATED ALWAYS AS (CAST(SUBSTR(occurrence_date, 6, 2) AS INTEGER)) STORED,
  occurrence_date TEXT NOT NULL,
  particulars TEXT NOT NULL DEFAULT '',
  amount INTEGER NOT NULL,
  channel TEXT NOT NULL,
  category TEXT NOT NULL DEFAULT 'none',
  sub_category TEXT NOT NULL DEFAULT 'none',
  memo TEXT NOT NULL DEFAULT '',
  mf_transaction_id INTEGER UNIQUE -- FK
);
CREATE INDEX IF NOT EXISTS idx_transaction_history_year_month ON transaction_history(occurrence_year, occurrence_month);