INSERT INTO transaction_history (
    date,
    description,
    amount,
    major_category,
    minor_category,
    memo,
    mf_transaction_id
  )
SELECT sub.date,
  sub.description,
  sub.amount,
  COALESCE(sub.new_major_category, 'none') AS major_category,
  COALESCE(sub.new_minor_category, 'none') AS minor_category,
  sub.memo,
  sub.mf_id
FROM (
    SELECT mf.id AS mf_id,
      mf.date,
      mf.description,
      mf.amount,
      mf.memo,
      cr.new_major_category,
      cr.new_minor_category,
      ROW_NUMBER() OVER (
        PARTITION BY mf.id
        ORDER BY cr.id ASC
      ) AS rule_rank
    FROM mf_transaction mf
      LEFT JOIN mf_transaction_categorization_rule cr ON (
        cr.mf_include IS NULL
        OR mf.include = cr.mf_include
      )
      AND (
        cr.mf_date_min IS NULL
        OR mf.date >= cr.mf_date_min
      )
      AND (
        cr.mf_date_max IS NULL
        OR mf.date <= cr.mf_date_max
      )
      AND (
        cr.mf_description_glob IS NULL
        OR mf.description GLOB cr.mf_description_glob
      )
      AND (
        cr.mf_amount_min IS NULL
        OR mf.amount >= cr.mf_amount_min
      )
      AND (
        cr.mf_amount_max IS NULL
        OR mf.amount <= cr.mf_amount_max
      )
      AND (
        cr.mf_financial_institution IS NULL
        OR mf.financial_institution = cr.mf_financial_institution
      )
      AND (
        cr.mf_major_category IS NULL
        OR mf.major_category = cr.mf_major_category
      )
      AND (
        cr.mf_minor_category IS NULL
        OR mf.minor_category = cr.mf_minor_category
      )
      AND (
        cr.mf_memo_glob IS NULL
        OR mf.memo GLOB cr.mf_memo_glob
      )
      AND (
        cr.mf_transfer IS NULL
        OR mf.transfer = cr.mf_transfer
      )
      LEFT JOIN transaction_history th ON mf.id = th.mf_transaction_id
    WHERE th.mf_transaction_id IS NULL
  ) sub
WHERE sub.rule_rank = 1;