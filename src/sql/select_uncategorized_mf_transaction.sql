SELECT mf.*
FROM categorized_mf_transaction ct
  JOIN mf_transaction mf ON ct.id = mf.id
WHERE ct.category = 'none';