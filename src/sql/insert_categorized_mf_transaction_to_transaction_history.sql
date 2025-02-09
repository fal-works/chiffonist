INSERT INTO transaction_history (
    date,
    description,
    amount,
    category,
    sub_category,
    memo,
    mf_transaction_id
  )
SELECT mf.date,
  mf.description,
  mf.amount,
  ct.category,
  ct.sub_category,
  mf.memo,
  ct.id
FROM categorized_mf_transaction ct
  JOIN mf_transaction mf ON ct.id = mf.id;