SELECT DISTINCT mf.financial_institution
FROM mf_transaction mf
  LEFT JOIN mapping_mf_financial_institution_to_channel mp ON mf.financial_institution = mp.mf_financial_institution
WHERE mp.channel IS NULL;