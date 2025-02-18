//! Command line argument parsing and pretty help pages

use crate::*;
use crate as fox;
use colored::*;

#[derive(Clone)]
struct Parameter {
    long: String,
    has_value: bool
}

/// Used to specify what CLI arguments the program may take.
#[derive(Clone)]
pub struct ArgumentsParser {
    required: Vec<Parameter>,
    optional: Vec<Parameter>,
}

impl ArgumentsParser {
    pub fn new() -> Self {
        Self {
            required: vec![],
            optional: vec![],
        }
    }

    /// Specify that this CLI argument must exist
    ///
    /// `long`: Name of the argument
    pub fn required<S: Into<String>>(mut self, long: S) -> Self {
        self.required.push(Parameter { long: long.into(), has_value: true });
        self.clone()
    }

    /// Specify that this CLI argument may exist
    ///
    /// `long`: Name of the argument
    /// `has_value`: If true, argument must be followed by a value, otherwise it's a flag
    pub fn optional<S: Into<String>>(mut self, long: S, has_value: bool) -> Self {
        self.optional.push(Parameter { long: long.into(), has_value });
        self.clone()
    }

    pub fn parse(self) -> Arguments {
        let cli_args = std::env::args().skip(1).collect::<Vec<String>>();
        let mut i = 0;
        let mut args: Vec<Argument> = vec![];
        let mut found_args: Vec<String> = vec![];

        let combined: Vec<Parameter> = self.required.clone().into_iter().chain(self.optional).collect();

        for cli_arg in &cli_args {
            sdebug!("Parsing {}", cli_arg);
            for param in &combined {
                if *param.long != *cli_arg {
                    continue;
                }

                sdebug!("{}", i);

                if param.has_value {
                    sdebug!("{} must have value", param.long);
                    match cli_args.get(i + 1) {
                        Some(_) => {
                            args.push(Argument {
                                name: cli_args[i].clone(),
                                value: Some(cli_args[i + 1].clone())
                            });

                            if found_args.contains(&cli_args[i]) {
                                critical!("Argument `{}` provided twice.", cli_args[i]);
                                std::process::exit(1);
                            }
                            else {
                                found_args.push(cli_args[i].clone());
                            }

                            sdebug!("inc (value read)");
                        },
                        None => {
                            scritical!("No value provided for argument `{}`", cli_args[i]);
                            std::process::exit(1);
                        }
                    }
                }
                else {
                    args.push(Argument {
                        name: cli_args[i].clone(),
                        value: None
                    });

                    if found_args.contains(&cli_args[i]) {
                        scritical!("Argument `{}` provided twice.", cli_args[i]);
                        std::process::exit(1);
                    }
                    else {
                        found_args.push(cli_args[i].clone());
                    }
                }
            }

            sdebug!("inc (end loop)");
            i += 1
        }

        for required_arg in self.required {
            if !found_args.contains(&required_arg.long) {
                scritical!("Missing required argument `{}`", required_arg.long);
                std::process::exit(1);
            }
        }

        Arguments { arguments: args }
    }
}

pub struct Argument {
    name: String,
    value: Option<String>,
}

/// Reader for values and presence of CLI arguments.
pub struct Arguments {
    arguments: Vec<Argument>
}

impl Arguments {
    /// Get the value of a CLI argument.
    /// Note that this will fail if your argument is a flag (value-less)
    ///
    /// Example: `./my_program --out_dir /dev/null
    pub fn get_value<S: Into<String>>(&self, name: S) -> Option<String> {
        let name = name.into();
        for arg in &self.arguments {
            if arg.name == name {
                if let Some(val) = &arg.value {
                    return Some(val.clone())
                }

                if let None = &arg.value {
                    scritical!("Tried to get the value of an argument ({}), but the argument is a flag. Did you mean to use `has_flag()`?", arg.name);
                    std::process::exit(1);
                }
            }
        }

        None

        // scritical!("Tried to find the value of an argument ({}) that was not specified for parsing", name);
        // std::process::exit(1);
    }

    /// Determine if a CLI flag is present
    /// Note that this will fail if the argument has a value
    ///
    /// Example: `./my_program --debug
    pub fn has_flag<S: Into<String>>(&self, name: S) -> bool {
        let name = name.into();
        for arg in &self.arguments {
            if arg.name == name {
                if let None = &arg.value {
                    return true
                }

                if let None = &arg.value {
                    scritical!("Tried to determine if a flag is present ({}), but the flag has a value. Did you mean to use `get_value()`?", arg.name);
                    std::process::exit(1);
                }
            }
        }

        false
    }
}
