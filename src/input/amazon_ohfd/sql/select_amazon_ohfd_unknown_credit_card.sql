SELECT DISTINCT src.credit_card
FROM amazon_ohfd src
  LEFT JOIN mapping_amazon_ohfd_credit_card_to_channel mp ON src.credit_card = mp.credit_card
WHERE mp.channel IS NULL;