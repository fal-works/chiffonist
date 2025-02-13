CREATE VIEW IF NOT EXISTS transaction_view AS
SELECT th.id,
  th.occurrence_year,
  th.occurrence_month,
  th.occurrence_date,
  th.particulars,
  th.amount,
  th.channel,
  chg.channel_group,
  th.category,
  th.sub_category
FROM transaction_history th
  INNER JOIN map_channel_group_to_channel chg ON th.channel = chg.channel;