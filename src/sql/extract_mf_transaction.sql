SELECT
  mf.id,
  mf.date,
  mf.description,
  mf.amount,
  mf.major_category,
  mf.minor_category,
  mf.memo
FROM mf_transaction mf
  LEFT JOIN transaction_history th ON mf.id = th.mf_transaction_id
WHERE mf.include = 1
  AND mf.transfer = 0
  AND th.mf_transaction_id IS NULL;