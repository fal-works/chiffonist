-- amount は符号を反転します。
INSERT INTO transaction_history (
    occurrence_date,
    particulars,
    amount,
    channel,
    category,
    sub_category,
    amazon_ohfd_id
  )
SELECT src.order_date,
  src.product_name,
  - src.amount,
  ct.channel,
  ct.category,
  ct.sub_category,
  ct.id
FROM categorized_amazon_ohfd ct
  JOIN amazon_ohfd src ON ct.id = src.id;