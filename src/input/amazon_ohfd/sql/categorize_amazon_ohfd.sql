/*
 amazon_ohfd テーブルのレコードのうち、
 transaction_history テーブルに未登録で、かつ
 mapping_amazon_ohfd_credit_card_to_channel テーブルに金融機関が登録済みのものについて、
 amazon_ohfd_manual_categorization, amazon_ohfd_categorization_rule テーブルに従って分類を行い、
 一時テーブル categorized_amazon_ohfd に保存します。
 */
INSERT INTO categorized_amazon_ohfd (id, channel, category, sub_category)
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
    SELECT src.id AS id,
      chmap.channel,
      mc.category AS category_manual,
      mc.sub_category AS sub_category_manual,
      cr.new_category AS category_auto,
      cr.new_sub_category AS sub_category_auto,
      ROW_NUMBER() OVER (
        PARTITION BY src.id
        ORDER BY cr.id ASC
      ) AS rule_rank
    FROM amazon_ohfd src
      INNER JOIN mapping_amazon_ohfd_credit_card_to_channel chmap ON (
        src.credit_card = chmap.credit_card
      )
      LEFT JOIN amazon_ohfd_manual_categorization mc ON (
        src.order_no = mc.order_no
        AND src.product_name = mc.product_name
      )
      LEFT JOIN amazon_ohfd_categorization_rule cr ON (
        cr.order_date_min IS NULL
        OR src.order_date >= cr.order_date_min
      )
      AND (
        cr.order_date_max IS NULL
        OR src.order_date <= cr.order_date_max
      )
      AND (
        cr.product_name_glob IS NULL
        OR src.product_name GLOB cr.product_name_glob
      )
      AND (
        cr.product_info_glob IS NULL
        OR src.product_info GLOB cr.product_info_glob
      )
      AND (
        cr.amount_min IS NULL
        OR src.amount >= cr.amount_min
      )
      AND (
        cr.amount_max IS NULL
        OR src.amount <= cr.amount_max
      )
      AND (
        cr.credit_card IS NULL
        OR src.credit_card = cr.credit_card
      )
      LEFT JOIN transaction_history th ON src.id = th.amazon_ohfd_id
    WHERE th.amazon_ohfd_id IS NULL
  ) q
WHERE q.rule_rank = 1;