SELECT src.order_date,
  src.order_no,
  src.product_name,
  src.product_info,
  src.amount,
  src.credit_card
FROM categorized_amazon_ohfd ct
  JOIN amazon_ohfd src ON ct.id = src.id
WHERE ct.category = 'none';