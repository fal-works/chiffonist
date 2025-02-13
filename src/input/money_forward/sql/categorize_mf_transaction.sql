/*
 mf_transaction テーブルのレコードのうち、
 transaction_history テーブルに未登録で、かつ
 mapping_mf_financial_institution_to_channel テーブルに金融機関が登録済みのものについて、
 mf_transaction_manual_categorization, mf_transaction_categorization_rule テーブルに従って分類を行い、
 一時テーブル categorized_mf_transaction に保存します。
 */
INSERT INTO categorized_mf_transaction (id, channel, category, sub_category)
SELECT q.id,
  q.channel,
  COALESCE(
    q.category_manual,
    q.category_auto,
    'none'
  ) AS category,
  COALESCE(
    q.sub_category_manual,
    q.sub_category_auto,
    'none'
  ) AS sub_category
FROM (
    SELECT mf.id AS id,
      chmap.channel,
      mc.category AS category_manual,
      mc.sub_category AS sub_category_manual,
      cr.new_category AS category_auto,
      cr.new_sub_category AS sub_category_auto,
      ROW_NUMBER() OVER (
        PARTITION BY mf.id
        ORDER BY cr.id ASC
      ) AS rule_rank
    FROM mf_transaction mf
      INNER JOIN mapping_mf_financial_institution_to_channel chmap ON (
        mf.financial_institution = chmap.mf_financial_institution
      )
      LEFT JOIN mf_transaction_manual_categorization mc ON (
        mf.mf_original_id = mc.mf_original_id
      )
      LEFT JOIN mf_transaction_categorization_rule cr ON (
        cr.mf_include_flag IS NULL
        OR mf.include_flag = cr.mf_include_flag
      )
      AND (
        cr.mf_occurrence_date_min IS NULL
        OR mf.occurrence_date >= cr.mf_occurrence_date_min
      )
      AND (
        cr.mf_occurrence_date_max IS NULL
        OR mf.occurrence_date <= cr.mf_occurrence_date_max
      )
      AND (
        cr.mf_particulars_glob IS NULL
        OR mf.particulars GLOB cr.mf_particulars_glob
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
        cr.mf_intermediate_category IS NULL
        OR mf.intermediate_category = cr.mf_intermediate_category
      )
      AND (
        cr.mf_memo_glob IS NULL
        OR mf.memo GLOB cr.mf_memo_glob
      )
      AND (
        cr.mf_transfer_flag IS NULL
        OR mf.transfer_flag = cr.mf_transfer_flag
      )
      LEFT JOIN transaction_history th ON mf.id = th.mf_transaction_id
    WHERE th.mf_transaction_id IS NULL
  ) q
WHERE q.rule_rank = 1;