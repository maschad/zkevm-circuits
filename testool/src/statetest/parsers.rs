



#[derive(Debug, Clone, Deserialize)]
pub struct NativeType(String);

impl LangIndependentType {
	pub fn parse(&mut self, as_str: &str) -> Result<NativeType> {
		match as_str {
			 Some(hex) => as_str.strip_prefix("0x") = s
		}
		if let Some(hex) = as_str.strip_prefix("0x") {
		Ok(Address::from_slice(
			&hex::decode(hex).context("parse_address")?,
		))
		} else {
		Ok(Address::from_slice(
			&hex::decode(as_str).context("parse_address")?,
		))
		}
	}
}