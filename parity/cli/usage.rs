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
				$field_s:ident : $typ_s:ty, display $default_s:expr, or $from_config_s:expr,
			)*
		}
		{
			$(
				CMD $subcommand:ident
				{
					$(
						CMD $subsubcommand:ident
						{
							$(
								ARG $subsubcommand_arg:ident : $typ_subsubcommand_arg:ty, $clap_subsubcommand_arg:expr,
							)*
							$(
								ARGM $subsubcommand_argm:ident : $typ_subsubcommand_argm:ty, $clap_subsubcommand_argm:expr,
							)*
						}
					)*

					$(
						ARG $subcommand_arg:ident : $typ_subcommand_arg:ty, $clap_subcommand_arg:expr,
					)*
					$(
						ARGM $subcommand_argm:ident : $typ_subcommand_argm:ty, $clap_subcommand_argm:expr,
					)*
				}
			)*
		}
		{
			$(
			[$group_name:expr]
				$(
					FLAG $field_flag_u:ident : $typ_flag_u:ty = $default_flag_u:expr, or $from_config_flag_u:expr, $usage_flag_u:expr,
				)*
				$(
					ARG $field_arg_u:ident : $typ_arg_u:ty = $default_arg_u:expr, or $from_config_arg_u:expr, $usage_arg_u:expr,
				)*
				$(
					ARGM $field_argm_u:ident : $typ_argm_u:ty = $default_argm_u:expr, or $from_config_argm_u:expr, $usage_argm_u:expr,
				)*
				$(
					ARG_OPTION $field_argo_u:ident : $innertyp_argo_u:ty = $default_argo_u:expr, or $from_config_argo_u:expr, $usage_argo_u:expr,
				)*
			)*
		}
		{
			$(
				FLAG $field_a_flag:ident : $typ_a_flag:ty,
			)*
			$(
				ARG_OPTION $field_a_arg:ident : $typ_a_arg:ty,
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
				pub $field_s: $typ_s,
			)*

			$(
				pub $subcommand: bool,

				$(
					pub $subsubcommand: bool,
					$(
						pub $subsubcommand_arg: Option<$typ_subsubcommand_arg>,
					)*
					$(
						pub $subsubcommand_argm: Option<Vec<$typ_subsubcommand_argm>>,
					)*
				)*

				$(
					pub $subcommand_arg: Option<$typ_subcommand_arg>,
				)*
				$(
					pub $subcommand_argm: Option<Vec<$typ_subcommand_argm>>,
				)*
			)*

			$(
				$(
					pub $field_flag_u: bool, // @TODO hardcoded
				)*
				$(
					pub $field_arg_u: $typ_arg_u,
				)*
				$(
					pub $field_argm_u: Vec<$typ_argm_u>,
				)*
				$(
					pub $field_argo_u: Option<$innertyp_argo_u>,
				)*
			)*

			$(
				pub $field_a_flag: bool, // @TODO hardcoded
			)*
			$(
				pub $field_a_arg: Option<$typ_a_arg>,
			)*
		}

		impl Default for Args {
			fn default() -> Self {
				Args {

					$(
						$field_s: Default::default(),
					)*

					$(
						$subcommand: Default::default(),
						$(
							$subsubcommand: Default::default(),
							$(
								$subsubcommand_arg: Default::default(),
							)*
							$(
								$subsubcommand_argm: Default::default(),
							)*
						)*

						$(
							$subcommand_arg: Default::default(),
						)*
						$(
							$subcommand_argm: Default::default(),
						)*
					)*

					$(
						$(
							$field_flag_u: Default::default(),
						)*
						$(
							$field_arg_u: Default::default(),
						)*
						$(
							$field_argm_u: Default::default(),
						)*
						$(
							$field_argo_u: Default::default(),
						)*
					)*

					$(
						$field_a_flag: Default::default(),
					)*

					$(
						$field_a_arg: Default::default(),
					)*
				}
			}
		}

		#[derive(Default, Debug, PartialEq, Clone, Deserialize)]
		struct RawArgs {
			$(
				$field_s: Option<$typ_s>,
			)*
			$(
				$subcommand: bool,
				
				$(
					$subsubcommand: bool,
					$(
						$subsubcommand_arg: Option<$typ_subsubcommand_arg>,
					)*
					$(
						$subsubcommand_argm: Option<Vec<$typ_subsubcommand_argm>>,
					)*
				)*

				$(
					$subcommand_arg: Option<$typ_subcommand_arg>,
				)*
				$(
					$subcommand_argm: Option<Vec<$typ_subcommand_argm>>,
				)*
			)*
			$(
				$(
					$field_flag_u: bool, // @TODO HARDCODED / REMOVE TYPE FROM MACRO CALL
				)*
				$(
					$field_arg_u: Option<$typ_arg_u>,
				)*
				$(
					$field_argm_u: Option<Vec<$typ_argm_u>>,
				)*
				$(
					$field_argo_u: Option<$innertyp_argo_u>,
				)*
			)*

			$(
				$field_a_flag: bool, // @TODO HARDCODED / REMOVE TYPE FROM MACRO CALL
			)*

			$(
				$field_a_arg: Option<$typ_a_arg>,
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
					args.$field_s = self.$field_s.or_else(|| $from_config_s(&config)).unwrap_or(None);
				)*
				$(
					args.$subcommand = self.$subcommand;

					$(
						args.$subsubcommand = self.$subsubcommand;
						$(
							args.$subsubcommand_arg = self.$subsubcommand_arg;
						)*
						$(
							args.$subsubcommand_argm = self.$subsubcommand_argm;
						)*
					)*

					$(
						args.$subcommand_arg = self.$subcommand_arg;
					)*
					$(
						args.$subcommand_argm = self.$subcommand_argm;
					)*
				)*

				$(
					$(
						// args.$field_flag_u = self.$field_flag_u.or_else(|| $from_config_flag_u(&config)).unwrap_or_else(|| $default_flag_u.into());

						// Presence of CLI switch || config || default
						args.$field_flag_u = self.$field_flag_u || $from_config_flag_u(&config).unwrap_or_else(|| $default_flag_u.into());
					)*
					$(
						args.$field_arg_u = self.$field_arg_u.or_else(|| $from_config_arg_u(&config)).unwrap_or_else(|| $default_arg_u.into());
					)*
					$(
						args.$field_argm_u = self.$field_argm_u.or_else(|| $from_config_argm_u(&config)).unwrap_or_else(|| $default_argm_u.into());
					)*
					$(
						args.$field_argo_u = self.$field_argo_u.or_else(|| $from_config_argo_u(&config)).or_else(|| $default_argo_u.into()); // before was:unwrap_or_else intead of .or_else
					)*
				)*
				$(
					args.$field_a_flag = self.$field_a_flag;
				)*
				$(
					args.$field_a_arg = self.$field_a_arg;
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
								SubCommand::with_name(&(stringify!($subcommand)[4..])) // @TODO remove () after &
								$(
									.subcommand(
										SubCommand::with_name(&(stringify!($subsubcommand)[stringify!($subcommand).len()+1..]))
										$(
											.arg($clap_subsubcommand_arg(Arg::with_name(&(stringify!($subsubcommand_arg)[stringify!($subsubcommand).len()+1..]))))
										)*
										$(
											.arg($clap_subsubcommand_argm(Arg::with_name(&(stringify!($subsubcommand_argm)[stringify!($subsubcommand).len()+1..])).multiple(true)))
										)*
									)
								)*
								$(
									.arg($clap_subcommand_arg(Arg::with_name(&(stringify!($subcommand_arg)[stringify!($subcommand).len()+1..]))))
								)*
								$(
									.arg($clap_subcommand_argm(Arg::with_name(&(stringify!($subcommand_argm)[stringify!($subcommand).len()+1..])).multiple(true)))
								)*
							)
						)*
						.args(&[
							$(
								$(
									Arg::from_usage($usage_arg_u),
								)*
								$(
									Arg::from_usage($usage_flag_u),
								)*
							)*
							$(
								Arg::with_name(&(stringify!($field_a_flag)[5..])).hidden(true),
							)*
							$(
								Arg::with_name(&(stringify!($field_a_arg)[4..])).takes_value(true).hidden(true),
							)*
						])
						.get_matches_safe()?;

				let mut raw_args : RawArgs = Default::default();
				$(
					$(
						raw_args.$field_arg_u = value_t!(matches, &stringify!($field_arg_u)[4..], $typ_arg_u).ok();
					)*
					$(
						raw_args.$field_argm_u = values_t!(matches, &stringify!($field_argm_u)[4..], $typ_argm_u).ok();
					)*
					$(
						raw_args.$field_argo_u = value_t!(matches, &stringify!($field_argo_u)[4..], $innertyp_argo_u).ok();
					)*
					$(
						raw_args.$field_flag_u = matches.is_present(&(stringify!($field_flag_u)[5..]));
					)*
				)*
				
				$(
					// Subcommand
					if let Some(submatches) = matches.subcommand_matches(&(stringify!($subcommand)[4..])) {
						raw_args.$subcommand = true;

						$(
							// Sub-subcommand
							if let Some(subsubmatches) = submatches.subcommand_matches(&(stringify!($subsubcommand)[stringify!($subcommand).len()+1..])) {
								raw_args.$subsubcommand = true;

								// Sub-subcommand arguments
								$(
									raw_args.$subsubcommand_arg = value_t!(subsubmatches, &stringify!($subsubcommand_arg)[stringify!($subsubcommand).len()+1..], $typ_subsubcommand_arg).ok();
								)*
								$(
									// might need to convert from values to vec
									raw_args.$subsubcommand_argm = values_t!(subsubmatches, &stringify!($subsubcommand_argm)[stringify!($subsubcommand).len()+1..], $typ_subsubcommand_argm).ok();
								)*
							}
							else {
								raw_args.$subsubcommand = false;
							}
						)*

						// Subcommand arguments
						$(
							raw_args.$subcommand_arg = value_t!(submatches, &stringify!($subcommand_arg)[stringify!($subcommand).len()+1..], $typ_subcommand_arg).ok();
						)*
						$(
							raw_args.$subcommand_argm = values_t!(submatches, &stringify!($subcommand_argm)[stringify!($subcommand).len()+1..], $typ_subcommand_argm).ok();
						)*
					}
					else {
						raw_args.$subcommand = false;
					}
				)*
				

				$(
					raw_args.$field_a_flag = matches.is_present(&(stringify!($field_a_flag)[5..]));
				)*
				$(
					raw_args.$field_a_arg = value_t!(matches, &stringify!($field_a_arg)[4..], $typ_a_arg).ok();
				)*
				

				Ok(raw_args)				
			}
		}
	};
}