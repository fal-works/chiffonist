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
  mf_transaction_id INTEGER UNIQUE, -- REFERENCES mf_transaction(id)
  amazon_ohfd_id INTEGER UNIQUE -- REFERENCES amazon_ohfd(id)
);
CREATE INDEX IF NOT EXISTS idx_transaction_history_year_month ON transaction_history (occurrence_year, occurrence_month);
CREATE INDEX IF NOT EXISTS idx_transaction_history_occurrence_date ON transaction_history (occurrence_date);
CREATE INDEX IF NOT EXISTS idx_transaction_history_channel ON transaction_history (channel);
CREATE INDEX IF NOT EXISTS idx_transaction_history_categories ON transaction_history (category, sub_category);