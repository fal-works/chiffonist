INSERT INTO transaction_history (
    date,
    description,
    amount,
    data_source,
    category,
    sub_category,
    memo,
    mf_transaction_id
  )
SELECT mf.date,
  mf.description,
  mf.amount,
  'unknown',
  ct.category,
  ct.sub_category,
  mf.memo,
  ct.id
FROM categorized_mf_transaction ct
  JOIN mf_transaction mf ON ct.id = mf.id;