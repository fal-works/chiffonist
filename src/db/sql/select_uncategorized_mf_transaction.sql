SELECT mf.include_flag,
  mf.occurrence_date,
  mf.particulars,
  mf.amount,
  mf.financial_institution,
  mf.major_category,
  mf.intermediate_category,
  mf.memo,
  mf.transfer_flag,
  mf.mf_original_id
FROM categorized_mf_transaction ct
  JOIN mf_transaction mf ON ct.id = mf.id
WHERE ct.category = 'none';