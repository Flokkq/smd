use std::time::Duration;

use crate::error::{
	Error,
	Result,
};

use log::debug;
use ureq::{
	Agent,
	AgentBuilder,
};

use crate::config::VendorConfig;

/// Handles communication with a specified vendor.
///
/// The `Vendor` struct facilitates interaction with a store that adheres to a
/// defined API specification. This includes, but is not limited to, the
/// [official store](https://smd.flokkq.com/store). Users can implement and integrate
/// their own custom store by following the documented API requirements.
///
/// Additionally, a suite of integration tests is available to verify
/// compatibility with any custom implementation. These tests ensure that the
/// store meets the expected behavior defined by the API specification.
pub struct Vendor<'a> {
	communicator:  Agent,
	configuration: &'a VendorConfig,
}

impl<'a> Vendor<'a> {
	/// Construct a new vendor
	pub fn build(config: &'a VendorConfig) -> Self {
		let agent = AgentBuilder::new()
			.timeout_read(Duration::from_secs(5))
			.timeout_write(Duration::from_secs(5))
			.build();

		Self {
			communicator:  agent,
			configuration: config,
		}
	}

	fn get(
		&mut self,
		store: Option<url::Url>,
		path: &str,
	) -> Result<ureq::Response> {
		let base_url = store
			.or_else(|| self.configuration.default_store.clone())
			.ok_or_else(|| Error::StoreMissingError)?;

		debug!("Fetching '{}{}'", base_url, path);

		Ok(self
			.communicator
			.get(&format!("{}{}", base_url, path))
			.call()
			.map_err(|err| Error::VendorError(err))?)
	}

	#[cfg(test)]
	pub fn health_check(&mut self, store: Option<url::Url>) -> Result<()> {
		let status_code = self.get(store, "health_check")?.status();

		(status_code == 200).then(|| ()).ok_or_else(|| {
			Error::CustomError(format!(
				"Health check failed with status code: {}",
				status_code
			))
		})
	}
}

#[cfg(test)]
mod tests {
	use std::str::FromStr;

	use url::Url;
	use wiremock::{
		matchers::path,
		Mock,
		MockServer,
		ResponseTemplate,
	};

	use crate::config::VendorConfig;

	use super::Vendor;

	#[tokio::test]
	async fn request_reaches_server() {
		let config = VendorConfig::default();
		let mut vendor = Vendor::build(&config);
		let mock_server = MockServer::start().await;

		Mock::given(wiremock::matchers::method("GET"))
			.and(path("/health_check"))
			.respond_with(ResponseTemplate::new(200))
			.mount(&mock_server)
			.await;

		let status = vendor.health_check(Some(
			Url::from_str(&mock_server.uri())
				.expect("Eror converting mock_server uri to address"),
		));

		assert!(status.is_ok())
	}
}
