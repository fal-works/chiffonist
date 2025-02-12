INSERT
  OR IGNORE INTO amazon_ohfd (
    order_date,
    order_no,
    product_name,
    product_info,
    amount,
    credit_card
  )
SELECT *
FROM amazon_ohfd_addition
ORDER BY order_date;