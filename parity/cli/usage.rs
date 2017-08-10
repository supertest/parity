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
								ARG $subc_subc_arg:ident : $subc_subc_arg_type:ty, $subc_subc_arg_usage:expr,
							)*
							$(
								ARG_MULTIPLE $subc_subc_argm:ident : $subc_subc_argm_type:ty, $subc_subc_argm_usage:expr,
							)*
						}
					)*

					$(
						ARG $subc_arg:ident : $subc_arg_type:ty, $subc_arg_usage:expr,
					)*
					$(
						ARG_MULTIPLE $subc_argm:ident : $subc_argm_type:ty, $subc_argm_usage:expr,
					)*
				}
			)*
		}
		{
			$(
			[$group_name:expr]
				$(
					FLAG $flag:ident : bool = $flag_default:expr, or $flag_from_config:expr, $flag_usage:expr, $flag_help:expr,
				)*
				$(
					ARG $arg:ident : $arg_type:ty = $arg_default:expr, or $arg_from_config:expr, $arg_usage:expr, $arg_help:expr,
				)*
				$(
					ARG_MULTIPLE $argm:ident : $argm_type:ty = $argm_default:expr, or $argm_from_config:expr, $argm_usage:expr, $argm_help:expr,
				)*
				$(
					ARG_OPTION $argo:ident : $argo_type:ty = $argo_default:expr, or $argo_from_config:expr, $argo_usage:expr, $argo_help:expr,
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

			// Rust issue #22630
			#[allow(unused_assignments)]
			pub fn print_help() -> String {
				let mut help : String = include_str!("./usage_header.txt").to_owned();

				help.push_str("\n\n");

				// Subcommands
				help.push_str("parity [options]\n");
				$(
					{
						let mut subc_subc_exist = false;

						$(
							subc_subc_exist = true;
							let subc_subc_arg_usages : Vec<&str> = vec![
								$(
									$subc_subc_arg_usage,
								)*
								$(
									$subc_subc_argm_usage,
								)*
							];

							if subc_subc_arg_usages.is_empty() {
								help.push_str(&format!("parity {} {} [options]\n", &str::replace(&stringify!($subc)[4..], "_", "-"), &str::replace(&stringify!($subc_subc)[stringify!($subc).len()+1..], "_", "-")));
							} else {
								help.push_str(&format!("parity {} {} {} [options]\n", &str::replace(&stringify!($subc)[4..], "_", "-"), &str::replace(&stringify!($subc_subc)[stringify!($subc).len()+1..], "_", "-"), subc_subc_arg_usages.join(" ")));
							}
						)*

						if !subc_subc_exist {
							let subc_arg_usages : Vec<&str> = vec![
								$(
									$subc_arg_usage,
								)*
								$(
									$subc_argm_usage,
								)*
							];

							if subc_arg_usages.is_empty() {
								help.push_str(&format!("parity {} [options]\n", &str::replace(&stringify!($subc)[4..], "_", "-")));
							} else {
								help.push_str(&format!("parity {} {} [options]\n", &str::replace(&stringify!($subc)[4..], "_", "-"), subc_arg_usages.join(" ")));
							}
						}
					}
				)*

				// Args and flags
				$(
					help.push_str("\n");
					help.push_str($group_name); help.push_str(":\n");

					$(
						help.push_str(&format!("\t{}\n\t\t{}\n", $flag_usage, $flag_help));
					)*

					$(
						help.push_str(&format!("\t{}\n\t\t{} (default: {})\n", $arg_usage, $arg_help, $arg_default));
					)*

					$(
						help.push_str(&format!("\t{}\n\t\t{} (default: {:?})\n", $argm_usage, $argm_help, {let x : Vec<$argm_type> = $argm_default; x}));
						// Vec::new:<String>
						// ou sinon { let X: Vec<String> = vec![] }
						// if $argm_default.is_empty() { "" } else { $argm_default.iter().map(|x| x.to_string()).join(", ") }));
					)*

					$(
						help.push_str(&format!("\t{}\n\t\t{} (default: {})\n", $argo_usage, $argo_help, $argo_default.unwrap_or("none")));
					)*

				)*

				help
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

				// Add the variable name as argument identifier
				// To do so, we have to specify if the argument is required; we default to optional
				let usages = vec![
					$(
						$(
							format!("[{}] {} '{}'",&stringify!($arg)[4..],$arg_usage,$arg_help),
						)*
						$(
							format!("[{}] {} '{}'",&stringify!($argm)[4..],$argm_usage,$argm_help),
						)*
						$(
							format!("[{}] {} '{}'",&stringify!($argo)[4..],$argo_usage,$argo_help),
						)*
						$(
							format!("[{}] {} '{}'",&stringify!($flag)[5..],$flag_usage,$flag_help),
						)*
					)*
				];

				let matches = App::new("Parity")
				    	.global_setting(AppSettings::VersionlessSubcommands)
				    	.global_setting(AppSettings::DeriveDisplayOrder)
				    	.global_setting(AppSettings::UnifiedHelpMessage)
						.help(Args::print_help().as_ref())
						.about(include_str!("./usage_header.txt")) // @TODO before_help()
						$(
							.subcommand(
								SubCommand::with_name(&str::replace(&stringify!($subc)[4..], "_", "-"))
								$(
									.subcommand(
										SubCommand::with_name(&str::replace(&stringify!($subc_subc)[stringify!($subc).len()+1..], "_", "-"))
										$(
											.arg(
													Arg::from_usage($subc_subc_arg_usage)
														// .with_name(&stringify!($subc_subc_arg)[stringify!($subc_subc).len()+1..])
														.long(str::replace(&stringify!($subc_subc_arg)[stringify!($subc_subc).len()+1..],"_","-").as_ref())
											)
										)*
										$(
											.arg(
													Arg::from_usage($subc_subc_argm_usage)
														// .with_name(&stringify!($subc_subc_argm)[stringify!($subc_subc).len()+1..])
														.long(str::replace(&stringify!($subc_subc_argm)[stringify!($subc_subc).len()+1..],"_","-").as_ref())
														.multiple(true)
											)
										)*
									)
								)*
								$(
									.arg(
											Arg::from_usage($subc_arg_usage)
												// .with_name(&stringify!($subc_arg)[stringify!($subc).len()+1..])
												.long(str::replace(&stringify!($subc_arg)[stringify!($subc).len()+1..],"_","-").as_ref())
									)
								)*
								$(
									.arg(
											Arg::from_usage($subc_argm_usage)
												// .with_name(&stringify!($subc_argm)[stringify!($subc).len()+1..])
												.long(str::replace(&stringify!($subc_argm)[stringify!($subc).len()+1..], "_", "-").as_ref())
												.multiple(true)
									)
								)*
							)
						)*
						.args(&usages.iter().map(|u| Arg::from_usage(u)).collect::<Vec<Arg>>())
						.args(&[
							$(
								Arg::with_name(&stringify!($legacy_flag)[5..])
									.long(str::replace(&stringify!($legacy_flag)[5..], "_", "-").as_ref()).hidden(true),
							)*
							$(
								Arg::with_name(&stringify!($legacy_arg)[4..])
									.long(str::replace(&stringify!($legacy_arg)[4..], "_", "-").as_ref()).takes_value(true).hidden(true),
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
					if let Some(submatches) = matches.subcommand_matches(&str::replace(&stringify!($subc)[4..], "_", "-")) {
						raw_args.$subc = true;

						$(
							// Sub-subcommand
							if let Some(subsubmatches) = submatches.subcommand_matches(&str::replace(&stringify!($subc_subc)[stringify!($subc).len()+1..], "_", "-")) {
								raw_args.$subc_subc = true;

								// Sub-subcommand arguments
								$(
									raw_args.$subc_subc_arg = value_t!(subsubmatches, &stringify!($subc_subc_arg)[stringify!($subc_subc).len()+1..], $subc_subc_arg_type).ok();
								)*
								$(
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

				// Parameter is the argument name (not the long version)
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
