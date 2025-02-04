INSERT INTO transaction_history (
    date,
    description,
    amount,
    -- major_category,
    -- minor_category,
    memo,
    mf_transaction_id
  )
SELECT m.date,
  m.description,
  m.amount,
  -- m.major_category,
  -- m.minor_category,
  m.memo,
  m.id
FROM mf_transaction m
  LEFT JOIN transaction_history t ON m.id = t.mf_transaction_id
WHERE m.include = 1
  AND m.transfer = 0
  AND t.mf_transaction_id IS NULL;