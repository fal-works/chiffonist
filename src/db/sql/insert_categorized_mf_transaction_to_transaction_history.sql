INSERT INTO transaction_history (
    occurrence_date,
    particulars,
    amount,
    channel,
    category,
    sub_category,
    memo,
    mf_transaction_id
  )
SELECT mf.occurrence_date,
  mf.particulars,
  mf.amount,
  ct.channel,
  ct.category,
  ct.sub_category,
  mf.memo,
  ct.id
FROM categorized_mf_transaction ct
  JOIN mf_transaction mf ON ct.id = mf.id;