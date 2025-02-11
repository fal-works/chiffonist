INSERT OR IGNORE INTO mf_transaction (
    include_flag,
    occurrence_date,
    particulars,
    amount,
    financial_institution,
    major_category,
    intermediate_category,
    memo,
    transfer_flag,
    mf_original_id
  )
SELECT *
FROM mf_transaction_addition
ORDER BY occurrence_date;