#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CliConfig {
    pub duration_seconds: Option<f64>,
}

impl CliConfig {
    pub fn parse<I, S>(args: I) -> Result<Self, String>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut duration_seconds = None;
        let mut args = args.into_iter();
        let _program_name = args.next();

        while let Some(argument) = args.next() {
            match argument.as_ref() {
                "--duration-seconds" => {
                    let value = args
                        .next()
                        .ok_or_else(|| "missing value for --duration-seconds".to_string())?;
                    let parsed = value
                        .as_ref()
                        .parse::<f64>()
                        .map_err(|_| format!("invalid duration seconds: {}", value.as_ref()))?;
                    duration_seconds = Some(parsed);
                }
                unknown => return Err(format!("unknown argument: {unknown}")),
            }
        }

        Ok(Self { duration_seconds })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_duration_seconds_flag() {
        let args = ["digital-synth", "--duration-seconds", "2.5"];

        let config = CliConfig::parse(args).expect("duration flag should parse");

        assert_eq!(config.duration_seconds, Some(2.5));
    }

    #[test]
    fn uses_no_duration_when_flag_is_absent() {
        let args = ["digital-synth"];

        let config = CliConfig::parse(args).expect("empty args should parse");

        assert_eq!(config.duration_seconds, None);
    }

    #[test]
    fn rejects_missing_duration_value() {
        let args = ["digital-synth", "--duration-seconds"];

        let error = CliConfig::parse(args).expect_err("missing duration should fail");

        assert_eq!(error, "missing value for --duration-seconds");
    }

    #[test]
    fn rejects_unknown_arguments() {
        let args = ["digital-synth", "--frequency", "220"];

        let error = CliConfig::parse(args).expect_err("unknown argument should fail");

        assert_eq!(error, "unknown argument: --frequency");
    }
}
