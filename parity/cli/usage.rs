// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

macro_rules! otry {
	($e: expr) => (
		match $e {
			Some(ref v) => v,
			None => {
				return None;
			}
		}
	)
}
macro_rules! usage {
	(
		{
			$(
				CMD $subc:ident
				{
					$(
						CMD $subc_subc:ident
						{
							$(
								ARG $subc_subc_arg:ident : $subc_subc_arg_type:ty, $subc_subc_arg_clap:expr,
							)*
							$(
								ARG_MULTIPLE $subc_subc_argm:ident : $subc_subc_argm_type:ty, $subc_subc_argm_clap:expr,
							)*
						}
					)*

					$(
						ARG $subc_arg:ident : $subc_arg_type:ty, $subc_arg_clap:expr,
					)*
					$(
						ARG_MULTIPLE $subc_argm:ident : $subc_argm_type:ty, $subc_argm_clap:expr,
					)*
				}
			)*
		}
		{
			$(
			[$group_name:expr]
				$(
					FLAG $flag:ident : bool = $flag_default:expr, or $flag_from_config:expr, $flag_usage:expr,
				)*
				$(
					ARG $arg:ident : $arg_type:ty = $arg_default:expr, or $arg_from_config:expr, $arg_usage:expr,
				)*
				$(
					ARG_MULTIPLE $argm:ident : $argm_type:ty = $argm_default:expr, or $argm_from_config:expr, $argm_usage:expr,
				)*
				$(
					ARG_OPTION $argo:ident : $argo_type:ty = $argo_default:expr, or $argo_from_config:expr, $argo_usage:expr,
				)*
			)*
		}
		{
			$(
				FLAG $legacy_flag:ident : bool,
			)*
			$(
				ARG_OPTION $legacy_arg:ident : $legacy_arg_type:ty,
			)*
		}
	) => {
		use toml;
		use std::{fs, io, process};
		use std::io::{Read, Write};
		use util::version;
		use clap::{Arg, App, SubCommand, AppSettings, Error as ClapError};
		use helpers::replace_home;

		#[derive(Debug)]
		pub enum ArgsError {
			Clap(ClapError),
			Decode(toml::de::Error),
			Config(String, io::Error),
		}

		impl ArgsError {
			pub fn exit(self) -> ! {
				match self {
					ArgsError::Clap(e) => e.exit(), // TODO PRINT ?
					ArgsError::Decode(e) => {
						println_stderr!("You might have supplied invalid parameters in config file.");
						println_stderr!("{}", e);
						process::exit(2)
					},
					ArgsError::Config(path, e) => {
						println_stderr!("There was an error reading your config file at: {}", path);
						println_stderr!("{}", e);
						process::exit(2)
					},
				}
			}
		}

		impl From<ClapError> for ArgsError {
			fn from(e: ClapError) -> Self {
				ArgsError::Clap(e)
			}
		}

		impl From<toml::de::Error> for ArgsError {
			fn from(e: toml::de::Error) -> Self {
				ArgsError::Decode(e)
			}
		}

		#[derive(Debug, PartialEq)]
		pub struct Args {
			$(
				pub $subc: bool,

				$(
					pub $subc_subc: bool,
					$(
						pub $subc_subc_arg: Option<$subc_subc_arg_type>,
					)*
					$(
						pub $subc_subc_argm: Option<Vec<$subc_subc_argm_type>>,
					)*
				)*

				$(
					pub $subc_arg: Option<$subc_arg_type>,
				)*
				$(
					pub $subc_argm: Option<Vec<$subc_argm_type>>,
				)*
			)*

			$(
				$(
					pub $flag: bool,
				)*
				$(
					pub $arg: $arg_type,
				)*
				$(
					pub $argm: Vec<$argm_type>,
				)*
				$(
					pub $argo: Option<$argo_type>,
				)*
			)*

			$(
				pub $legacy_flag: bool,
			)*
			$(
				pub $legacy_arg: Option<$legacy_arg_type>,
			)*
		}

		impl Default for Args {
			fn default() -> Self {
				Args {
					$(
						$subc: Default::default(),
						$(
							$subc_subc: Default::default(),
							$(
								$subc_subc_arg: Default::default(),
							)*
							$(
								$subc_subc_argm: Default::default(),
							)*
						)*

						$(
							$subc_arg: Default::default(),
						)*
						$(
							$subc_argm: Default::default(),
						)*
					)*

					$(
						$(
							$flag: Default::default(),
						)*
						$(
							$arg: Default::default(),
						)*
						$(
							$argm: Default::default(),
						)*
						$(
							$argo: Default::default(),
						)*
					)*

					$(
						$legacy_flag: Default::default(),
					)*

					$(
						$legacy_arg: Default::default(),
					)*
				}
			}
		}

		#[derive(Default, Debug, PartialEq, Clone, Deserialize)]
		struct RawArgs {
			$(
				$subc: bool,
				
				$(
					$subc_subc: bool,
					$(
						$subc_subc_arg: Option<$subc_subc_arg_type>,
					)*
					$(
						$subc_subc_argm: Option<Vec<$subc_subc_argm_type>>,
					)*
				)*

				$(
					$subc_arg: Option<$subc_arg_type>,
				)*
				$(
					$subc_argm: Option<Vec<$subc_argm_type>>,
				)*
			)*
			$(
				$(
					$flag: bool,
				)*
				$(
					$arg: Option<$arg_type>,
				)*
				$(
					$argm: Option<Vec<$argm_type>>,
				)*
				$(
					$argo: Option<$argo_type>,
				)*
			)*

			$(
				$legacy_flag: bool,
			)*

			$(
				$legacy_arg: Option<$legacy_arg_type>,
			)*
		}

		impl Args {

			pub fn parse<S: AsRef<str>>(command: &[S]) -> Result<Self, ArgsError> {
				let raw_args = RawArgs::parse(command)?;

				// Skip loading config file if no_config flag is specified
				if raw_args.flag_no_config {
					return Ok(raw_args.into_args(Config::default()));
				}

				let config_file = raw_args.arg_config.clone().unwrap_or_else(|| raw_args.clone().into_args(Config::default()).arg_config);
				let config_file = replace_home(&::dir::default_data_path(), &config_file);
				match (fs::File::open(&config_file), raw_args.arg_config.clone()) {
					// Load config file
					(Ok(mut file), _) => {
						println_stderr!("Loading config file from {}", &config_file);
						let mut config = String::new();
						file.read_to_string(&mut config).map_err(|e| ArgsError::Config(config_file, e))?;
						Ok(raw_args.into_args(Self::parse_config(&config)?))
					},
					// Don't display error in case default config cannot be loaded.
					(Err(_), None) => Ok(raw_args.into_args(Config::default())),
					// Config set from CLI (fail with error)
					(Err(_), Some(ref config_arg)) => {
						match presets::preset_config_string(config_arg) {
							Ok(s) => Ok(raw_args.into_args(Self::parse_config(&s)?)),
							Err(e) => Err(ArgsError::Config(config_file, e))
						}
					},
				}
			}

			#[cfg(test)]
			pub fn parse_without_config<S: AsRef<str>>(command: &[S]) -> Result<Self, ArgsError> {
				Self::parse_with_config(command, Config::default())
			}

			#[cfg(test)]
			fn parse_with_config<S: AsRef<str>>(command: &[S], config: Config) -> Result<Self, ArgsError> {
				RawArgs::parse(command).map(|raw| raw.into_args(config)).map_err(ArgsError::Clap)
			}

			fn parse_config(config: &str) -> Result<Config, ArgsError> {
				Ok(toml::from_str(config)?)
			}

			pub fn print_version() -> String {
				format!(include_str!("./version.txt"), version())
			}
		}

		impl RawArgs {
			fn into_args(self, config: Config) -> Args {
				let mut args = Args::default();
				$(
					args.$subc = self.$subc;

					$(
						args.$subc_subc = self.$subc_subc;
						$(
							args.$subc_subc_arg = self.$subc_subc_arg;
						)*
						$(
							args.$subc_subc_argm = self.$subc_subc_argm;
						)*
					)*

					$(
						args.$subc_arg = self.$subc_arg;
					)*
					$(
						args.$subc_argm = self.$subc_argm;
					)*
				)*

				$(
					$(
						// args.$flag = self.$flag.or_else(|| $flag_from_config(&config)).unwrap_or_else(|| $flag_default.into());

						// Presence of CLI switch || config || default
						args.$flag = self.$flag || $flag_from_config(&config).unwrap_or_else(|| $flag_default.into());
					)*
					$(
						args.$arg = self.$arg.or_else(|| $arg_from_config(&config)).unwrap_or_else(|| $arg_default.into());
					)*
					$(
						args.$argm = self.$argm.or_else(|| $argm_from_config(&config)).unwrap_or_else(|| $argm_default.into());
					)*
					$(
						args.$argo = self.$argo.or_else(|| $argo_from_config(&config)).or_else(|| $argo_default.into()); // before was:unwrap_or_else intead of .or_else
					)*
				)*
				$(
					args.$legacy_flag = self.$legacy_flag;
				)*
				$(
					args.$legacy_arg = self.$legacy_arg;
				)*
				args
			}

			#[allow(unused_variables)] // when there are no subcommand args, the submatches aren't used
			pub fn parse<S: AsRef<str>>(command: &[S]) -> Result<Self, ClapError> {

				let matches = App::new("Parity")
				    	.setting(AppSettings::VersionlessSubcommands)
				    	.setting(AppSettings::DeriveDisplayOrder)
				    	.setting(AppSettings::UnifiedHelpMessage)
						.arg(Arg::with_name("version")
							.short("v")
							.long("version")
							.help(&Args::print_version()))
						.about(include_str!("./usage_header.txt"))
						$(
							.subcommand(
								SubCommand::with_name(&stringify!($subc)[4..]) // @TODO remove () after &
								$(
									.subcommand(
										SubCommand::with_name(&stringify!($subc_subc)[stringify!($subc).len()+1..])
										$(
											.arg($subc_subc_arg_clap(Arg::with_name(&stringify!($subc_subc_arg)[stringify!($subc_subc).len()+1..])))
										)*
										$(
											.arg($subc_subc_argm_clap(Arg::with_name(&stringify!($subc_subc_argm)[stringify!($subc_subc).len()+1..]).multiple(true)))
										)*
									)
								)*
								$(
									.arg($subc_arg_clap(Arg::with_name(&stringify!($subc_arg)[stringify!($subc).len()+1..])))
								)*
								$(
									.arg($subc_argm_clap(Arg::with_name(&stringify!($subc_argm)[stringify!($subc).len()+1..]).multiple(true)))
								)*
							)
						)*
						.args(&[
							$(
								$(
									Arg::from_usage($arg_usage),
								)*
								$(
									Arg::from_usage($flag_usage),
								)*
							)*
							$(
								Arg::with_name(&stringify!($legacy_flag)[5..]).hidden(true),
							)*
							$(
								Arg::with_name(&stringify!($legacy_arg)[4..]).takes_value(true).hidden(true),
							)*
						])
						.get_matches_safe()?;

				let mut raw_args : RawArgs = Default::default();
				$(
					$(
						raw_args.$arg = value_t!(matches, &stringify!($arg)[4..], $arg_type).ok();
					)*
					$(
						raw_args.$argm = values_t!(matches, &stringify!($argm)[4..], $argm_type).ok();
					)*
					$(
						raw_args.$argo = value_t!(matches, &stringify!($argo)[4..], $argo_type).ok();
					)*
					$(
						raw_args.$flag = matches.is_present(&stringify!($flag)[5..]);
					)*
				)*
				
				$(
					// Subcommand
					if let Some(submatches) = matches.subcommand_matches(&stringify!($subc)[4..]) {
						raw_args.$subc = true;

						$(
							// Sub-subcommand
							if let Some(subsubmatches) = submatches.subcommand_matches(&stringify!($subc_subc)[stringify!($subc).len()+1..]) {
								raw_args.$subc_subc = true;

								// Sub-subcommand arguments
								$(
									raw_args.$subc_subc_arg = value_t!(subsubmatches, &stringify!($subc_subc_arg)[stringify!($subc_subc).len()+1..], $subc_subc_arg_type).ok();
								)*
								$(
									// might need to convert from values to vec
									raw_args.$subc_subc_argm = values_t!(subsubmatches, &stringify!($subc_subc_argm)[stringify!($subc_subc).len()+1..], $subc_subc_argm_type).ok();
								)*
							}
							else {
								raw_args.$subc_subc = false;
							}
						)*

						// Subcommand arguments
						$(
							raw_args.$subc_arg = value_t!(submatches, &stringify!($subc_arg)[stringify!($subc).len()+1..], $subc_arg_type).ok();
						)*
						$(
							raw_args.$subc_argm = values_t!(submatches, &stringify!($subc_argm)[stringify!($subc).len()+1..], $subc_argm_type).ok();
						)*
					}
					else {
						raw_args.$subc = false;
					}
				)*
				

				$(
					raw_args.$legacy_flag = matches.is_present(&stringify!($legacy_flag)[5..]);
				)*
				$(
					raw_args.$legacy_arg = value_t!(matches, &stringify!($legacy_arg)[4..], $legacy_arg_type).ok();
				)*
				

				Ok(raw_args)				
			}
		}
	};
}