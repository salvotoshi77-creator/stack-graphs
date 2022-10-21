// -*- coding: utf-8 -*-
// ------------------------------------------------------------------------------------------------
// Copyright © 2022, stack-graphs authors.
// Licensed under either of Apache License, Version 2.0, or MIT license, at your option.
// Please see the LICENSE-APACHE or LICENSE-MIT files in this distribution for license details.
// ------------------------------------------------------------------------------------------------

//! Defines CLI

pub(self) const MAX_PARSE_ERRORS: usize = 5;

pub mod init;
pub mod load;
pub mod parse;
pub mod test;
mod util;

pub use path_loading::Cli as PathLoadingCli;
pub use provided_languages::Cli as ProvidedLanguagesCli;

mod path_loading {
    use anyhow::Result;
    use clap::Parser;
    use clap::Subcommand;

    use crate::cli::init::InitArgs;
    use crate::cli::load::PathLoadArgs;
    use crate::cli::parse::ParseArgs;
    use crate::cli::test::TestArgs;

    /// CLI implementation that loads grammars and stack graph definitions from paths.
    #[derive(Parser)]
    #[clap(about, version)]
    pub struct Cli {
        #[clap(subcommand)]
        command: Commands,
    }

    impl Cli {
        pub fn main() -> Result<()> {
            let cli = Cli::parse();
            match &cli.command {
                Commands::Init(cmd) => cmd.run(),
                Commands::Parse(cmd) => cmd.run(),
                Commands::Test(cmd) => cmd.run(),
            }
        }
    }

    #[derive(Subcommand)]
    enum Commands {
        Init(Init),
        Parse(Parse),
        Test(Test),
    }

    /// Init command
    #[derive(clap::Parser)]
    pub struct Init {
        #[clap(flatten)]
        init_args: InitArgs,
    }

    impl Init {
        pub fn run(&self) -> anyhow::Result<()> {
            self.init_args.run()
        }
    }

    /// Parse command
    #[derive(clap::Parser)]
    pub struct Parse {
        #[clap(flatten)]
        load_args: PathLoadArgs,
        #[clap(flatten)]
        parse_args: ParseArgs,
    }

    impl Parse {
        pub fn run(&self) -> anyhow::Result<()> {
            let mut loader = self.load_args.new_loader()?;
            self.parse_args.run(&mut loader)
        }
    }

    /// Test command
    #[derive(clap::Parser)]
    pub struct Test {
        #[clap(flatten)]
        load_args: PathLoadArgs,
        #[clap(flatten)]
        test_args: TestArgs,
    }

    impl Test {
        pub fn run(&self) -> anyhow::Result<()> {
            let mut loader = self.load_args.new_loader()?;
            self.test_args.run(&mut loader)
        }
    }
}

mod provided_languages {
    use anyhow::Result;
    use clap::Parser;
    use clap::Subcommand;

    use crate::cli::parse::ParseArgs;
    use crate::cli::test::TestArgs;
    use crate::loader::LanguageConfiguration;
    use crate::loader::Loader;

    /// CLI implementation that loads from provided grammars and stack graph definitions.
    #[derive(Parser)]
    #[clap(about, version)]
    pub struct Cli {
        #[clap(subcommand)]
        command: Commands,
    }

    impl Cli {
        pub fn main(configurations: Vec<LanguageConfiguration>) -> Result<()> {
            let cli = Cli::parse();
            let mut loader = Loader::from_language_configurations(configurations)?;
            match &cli.command {
                Commands::Parse(cmd) => cmd.run(&mut loader),
                Commands::Test(cmd) => cmd.run(&mut loader),
            }
        }
    }

    #[derive(Subcommand)]
    enum Commands {
        Parse(Parse),
        Test(Test),
    }

    /// Parse command
    #[derive(clap::Parser)]
    pub struct Parse {
        #[clap(flatten)]
        parse_args: ParseArgs,
    }

    impl Parse {
        pub fn run(&self, loader: &mut Loader) -> anyhow::Result<()> {
            self.parse_args.run(loader)
        }
    }

    /// Test command
    #[derive(clap::Parser)]
    pub struct Test {
        #[clap(flatten)]
        test_args: TestArgs,
    }

    impl Test {
        pub fn run(&self, loader: &mut Loader) -> anyhow::Result<()> {
            self.test_args.run(loader)
        }
    }
}
