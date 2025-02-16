/// `YYYY/MM/DD` または `Y/M/D` 形式の年月日の文字列を `YYYY-MM-DD` 形式に変換します。
/// 上記以外の形式であれば `Err` を返します。
pub fn normalize_slashed_date(input: &str) -> Result<String, String> {
  let parts: Vec<&str> = input.split('/').collect();

  if parts.len() != 3 {
      return Err(format!("Invalid date format: {}", input));
  }

  let year = parts[0]
      .parse::<u32>()
      .map_err(|_| format!("Invalid year: {}", parts[0]))?;
  let month = parts[1]
      .parse::<u32>()
      .map_err(|_| format!("Invalid month: {}", parts[1]))?;
  let day = parts[2]
      .parse::<u32>()
      .map_err(|_| format!("Invalid day: {}", parts[2]))?;

  if !(1..=9999).contains(&year) {
      return Err(format!("Year out of range: {}", year));
  }
  if !(1..=12).contains(&month) {
      return Err(format!("Month out of range: {}", month));
  }
  if !(1..=31).contains(&day) {
      return Err(format!("Day out of range: {}", day));
  }

  Ok(format!("{:04}-{:02}-{:02}", year, month, day))
}

/// 文字数を切り詰めます。
/// 切り詰め発生時には末尾に `"..."` を付与します。
/// 簡易的に、ASCII文字以外は2文字としてカウントします。
pub fn truncate_string(s: &str, max_width: usize) -> String {
  let mut width = 0;
  let mut result = String::new();

  for ch in s.chars() {
      let char_width = if ch.is_ascii() { 1 } else { 2 };
      if width + char_width > max_width {
          result.push_str("...");
          break;
      }
      result.push(ch);
      width += char_width;
  }

  result
}
