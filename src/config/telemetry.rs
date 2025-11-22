use std::{
    env::VarError,
    error::Error as _,
    fmt::{self, Display},
    io::IsTerminal,
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use tracing::Subscriber;
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    EnvFilter, Layer, filter::Directive, fmt::Layer as FmtLayer, layer::SubscriberExt,
    registry::LookupSpan, util::SubscriberInitExt,
};

use super::{ConfigError, ConfigResult};

/// Logging level configuration.
///
/// Determines the minimum severity level for log messages to be recorded.
/// Can be configured via YAML or environment variables.
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub enum Level {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "trace")]
    Trace,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    #[default]
    Info,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "error")]
    Error,
}

impl Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Off => "off",
                Self::Trace => "trace",
                Self::Debug => "debug",
                Self::Info => "info",
                Self::Warn => "warn",
                Self::Error => "error",
            }
        )
    }
}

/// Log output format configuration.
///
/// Determines how log messages are formatted when written to output.
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub enum Format {
    #[serde(rename = "compact")]
    Compact,
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "pretty")]
    #[default]
    Pretty,
}

impl Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Compact => "compact",
                Self::Full => "full",
                Self::Json => "json",
                Self::Pretty => "pretty",
            }
        )
    }
}

/// Logger configuration for the application.
///
/// Configures the tracing subscriber with the specified level, format,
/// and per-crate log directives. Supports environment variable overrides
/// via `RUST_LOG`.
#[derive(Debug, Deserialize, Clone)]
pub struct Logger {
    level: Level,
    format: Format,
    crates: Vec<String>,
}

impl Logger {
    /// Initializes the global tracing subscriber with the configured settings.
    ///
    /// Sets up the tracing subscriber with the specified:
    /// - [`Format`],
    /// - [`Level`] and
    /// - per-crate directives.
    ///
    /// This should be called once at application startup.
    ///
    /// ## Errors
    ///
    /// * Environment filter parsing errors
    /// * Invalid log directive format
    /// * Subscriber already initialized
    pub fn setup(&self) -> ConfigResult<()> {
        let env_filter_layer = self.env_filter()?;
        let registry = tracing_subscriber::registry()
            .with(env_filter_layer)
            .with(ErrorLayer::default());

        match self.format {
            Format::Compact => registry.with(self.compact_fmt_layer()).try_init()?,
            Format::Full => registry.with(self.base_fmt_layer()).try_init()?,
            Format::Json => registry.with(self.json_fmt_layer()).try_init()?,
            Format::Pretty => registry.with(self.pretty_fmt_layer()).try_init()?,
        }

        Ok(())
    }

    /// Creates an [`EnvFilter`] from configuration and environment variables.
    ///
    /// Checks for `RUST_LOG` environment variable first. If not present, uses
    /// the configured level and crate directives.
    ///
    /// ## Errors
    /// * Invalid `RUST_LOG` environment variable format
    /// * Invalid directive format in configuration
    fn env_filter(&self) -> ConfigResult<EnvFilter> {
        let mut env_filter: EnvFilter = match EnvFilter::try_from_default_env() {
            Ok(env_filter) => env_filter,
            Err(from_env_err) => {
                if let Some(err) = from_env_err.source() {
                    match err.downcast_ref::<VarError>() {
                        Some(VarError::NotPresent) => (),
                        Some(other) => return Err(ConfigError::EnvFilter(other.clone())), // Converts into crate::Report
                        _ => return Err(ConfigError::FromEnv(from_env_err)),
                    }
                }

                if self.crates.is_empty() {
                    EnvFilter::try_new(format!("{}={}", env!("CARGO_PKG_NAME"), &self.level))?
                } else {
                    EnvFilter::try_new("")?
                }
            }
        };

        let directives = self.directives()?;

        for directive in directives {
            env_filter = env_filter.add_directive(directive);
        }

        Ok(env_filter)
    }

    #[allow(clippy::unused_self)]
    fn base_fmt_layer<S>(&self) -> FmtLayer<S>
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        FmtLayer::new()
            .with_ansi(std::io::stderr().is_terminal())
            // TODO: Implement other writers
            .with_writer(std::io::stdout)
    }

    fn pretty_fmt_layer<S>(&self) -> impl Layer<S>
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        self.base_fmt_layer().pretty()
    }

    fn json_fmt_layer<S>(&self) -> impl Layer<S>
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        self.base_fmt_layer().json()
    }

    fn compact_fmt_layer<S>(&self) -> impl Layer<S>
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        self.base_fmt_layer()
            .compact()
            .with_target(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_file(false)
            .with_line_number(false)
    }

    #[must_use]
    pub fn level(&self) -> &Level {
        &self.level
    }

    #[must_use]
    pub fn format(&self) -> &Format {
        &self.format
    }

    /// Converts the configured crates list into tracing [`Directive`]
    ///
    /// Creates a directive for each crate in the format `crate=level`,
    /// using the configured log level.
    ///
    /// ## Errors
    /// * Invalid directive format (malformed crate name)
    pub fn directives(&self) -> ConfigResult<Vec<Directive>> {
        self.crates
            .iter()
            .map(|c: &String| -> ConfigResult<Directive> {
                let str_directive: String = format!("{}={}", c, &self.level);
                Ok(Directive::from_str(&str_directive)?)
            })
            .collect()
    }
}
