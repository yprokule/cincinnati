//! Options shared by CLI and TOML.

use super::AppSettings;
use commons::{parse_params_set, parse_path_prefix, MergeOptions};
use std::collections::HashSet;
use std::net::IpAddr;

/// Status service options.
#[derive(Debug, Deserialize, Serialize, StructOpt)]
pub struct StatusOptions {
    /// Address on which the status service will listen
    #[structopt(name = "status_address", long = "status.address")]
    pub address: Option<IpAddr>,

    /// Port to which the status service will bind
    #[structopt(name = "status_port", long = "status.port")]
    pub port: Option<u16>,
}

impl MergeOptions<Option<StatusOptions>> for AppSettings {
    fn merge(&mut self, opts: Option<StatusOptions>) {
        if let Some(status) = opts {
            assign_if_some!(self.status_address, status.address);
            assign_if_some!(self.status_port, status.port);
        }
    }
}

/// Options for the main Cincinnati service.
#[derive(Debug, Deserialize, Serialize, StructOpt)]
pub struct ServiceOptions {
    /// Address on which the server will listen
    #[structopt(name = "service_address", long = "service.address")]
    pub address: Option<IpAddr>,

    /// Port to which the server will bind
    #[structopt(name = "service_port", long = "service.port")]
    pub port: Option<u16>,

    /// Namespace prefix for all service endpoints (e.g. '/<prefix>/v1/graph')
    #[structopt(long = "service.path_prefix", parse(from_str = "parse_path_prefix"))]
    pub path_prefix: Option<String>,

    /// Comma-separated set of mandatory client parameters
    #[structopt(
        long = "service.mandatory_client_parameters",
        parse(from_str = "parse_params_set")
    )]
    pub mandatory_client_parameters: Option<HashSet<String>>,
}

impl MergeOptions<Option<ServiceOptions>> for AppSettings {
    fn merge(&mut self, opts: Option<ServiceOptions>) {
        if let Some(service) = opts {
            assign_if_some!(self.address, service.address);
            assign_if_some!(self.port, service.port);
            assign_if_some!(self.path_prefix, service.path_prefix);
            if let Some(params) = service.mandatory_client_parameters {
                self.mandatory_client_parameters.extend(params);
            }
        }
    }
}

/// Options for a Cincinnati upstream.
#[derive(Debug, Deserialize, StructOpt)]
pub struct UpCincinnatiOptions {
    /// Base URL for the upstream Cincinnati
    #[structopt(long = "upstream.cincinnati.url", parse(try_from_str = "uri_from_str"))]
    #[serde(default = "Option::default", deserialize_with = "de_uri")]
    pub url: Option<hyper::Uri>,
}

impl MergeOptions<Option<UpCincinnatiOptions>> for AppSettings {
    fn merge(&mut self, opts: Option<UpCincinnatiOptions>) {
        if let Some(up) = opts {
            assign_if_some!(self.upstream, up.url);
        }
    }
}

/// Parse a URI from a string.
pub fn uri_from_str<S>(input: S) -> failure::Fallible<hyper::Uri>
where
    S: AsRef<str>,
{
    let uri: hyper::Uri = input.as_ref().parse()?;
    Ok(uri)
}

/// Deserialize a URI from a string value.
pub fn de_uri<'de, D>(deserializer: D) -> Result<Option<hyper::Uri>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    use serde::Deserialize;

    let input = String::deserialize(deserializer)?;
    let uri: hyper::Uri = input.parse().map_err(D::Error::custom)?;
    Ok(Some(uri))
}