-- Chrome拡張「アマゾン注文履歴フィルタ」
-- ( https://chromewebstore.google.com/detail/アマゾン注文履歴フィルタ/jaikhcpoplnhinlglnkmihfdlbamhgig )
-- から出力されたデジタルコンテンツ注文一覧のCSVのうち主要なカラムのデータを保存します。
CREATE TABLE IF NOT EXISTS amazon_ohfd (
  id INTEGER NOT NULL PRIMARY KEY,
  order_date TEXT NOT NULL,
  order_no TEXT NOT NULL,
  product_name TEXT NOT NULL,
  product_info TEXT NOT NULL,
  amount INTEGER NOT NULL,
  credit_card TEXT
);
CREATE INDEX IF NOT EXISTS idx_amazon_ohf_order_date ON amazon_ohfd (order_date);
CREATE INDEX IF NOT EXISTS idx_amazon_ohf_credit_card ON amazon_ohfd (credit_card);