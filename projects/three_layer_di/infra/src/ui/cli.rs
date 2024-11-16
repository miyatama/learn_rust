use crate::{id_generator::IdGenerator, repository_impls::RepositoryImpls};
use clap::{Args, Parser, Subcommand, ValueEnum};
use domain::{EmailAddress, User, UserFirstName, UserId, UserLastName, UserName};
use interface_adapter::{
    AddUserRequestDTO, Controller, SearchUsersRequestDTO, UpdateUserRequestDTO,
};

#[derive(Parser, Debug)]
#[command(version, about)]
struct AppArgs {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Searches users by name and/or email address")]
    Search {
        #[arg(short, long, value_name = "firstname")]
        firstname: Option<String>,

        #[arg(short, long, value_name = "lastname")]
        lastname: Option<String>,

        #[arg(short, long, value_name = "email")]
        email: Option<String>,
    },

    #[command(about = "Adds a user")]
    Add {
        #[arg(short, long, require_equals = true, value_name = "firstname")]
        firstname: String,

        #[arg(short, long, require_equals = true, value_name = "lastname")]
        lastname: String,

        #[arg(short, long, require_equals = true, value_name = "email")]
        email: String,
    },

    #[command(about = "Updates a user's name")]
    Update {
        #[arg(short, long, value_name = "firstname")]
        firstname: Option<String>,

        #[arg(short, long, value_name = "lastname")]
        lastname: Option<String>,

        #[arg(short, long, require_equals = true, value_name = "email")]
        email: String,
    },
}

pub(crate) struct Cli<'r> {
    controller: Controller<'r, RepositoryImpls>,
}

impl<'r> Cli<'r> {
    pub fn new(repositories: &'r RepositoryImpls) -> Self {
        let controller = Controller::new(repositories);
        Self { controller }
    }

    pub(crate) fn process_cmd(&self) {
        let appArgs = Self::create_matches();

        if let Some(command) = appArgs.command {
            match command {
                Commands::Search {
                    firstname,
                    lastname,
                    email,
                } => {
                    self.process_search_cmd(firstname, lastname, email);
                }
                Commands::Add {
                    firstname,
                    lastname,
                    email,
                } => {
                    self.process_add_cmd(firstname, lastname, email);
                }
                Commands::Update {
                    firstname,
                    lastname,
                    email,
                } => {
                    self.process_update_cmd(firstname, lastname, email);
                }
            }
        } else {
            panic!("Invalid command. Run with --help for usage.")
        }
    }

    fn process_search_cmd(
        &self,
        firstname: Option<String>,
        lastname: Option<String>,
        email: Option<String>,
    ) {
        let req = SearchUsersRequestDTO {
            email: email.map(EmailAddress::new),
            first_name: firstname.map(UserFirstName::new),
            last_name: lastname.map(UserLastName::new),
        };

        let res = self.controller.search_users(req);
        eprintln!("Found users:\n{:#?}", res.users)
    }

    fn process_add_cmd(&self, firstname: String, lastname: String, email: String) {
        let id: u64 = IdGenerator::gen();

        let user = User::new(
            UserId::new(id),
            UserName::new(UserFirstName::new(firstname), UserLastName::new(lastname)),
            EmailAddress::new(email),
        );
        let req = AddUserRequestDTO { user };

        match self.controller.add_user(req) {
            Ok(_res) => {
                eprintln!("Successfully added a user.")
            }
            Err(e) => {
                // TODO 丁寧なエラーハンドリング
                eprintln!("Failed to add a user: {:?}", e)
            }
        }
    }
    fn process_update_cmd(
        &self,
        firstname: Option<String>,
        lastname: Option<String>,
        email: String,
    ) {
        let req = UpdateUserRequestDTO {
            email: EmailAddress::new(email),
            first_name: firstname.map(UserFirstName::new),
            last_name: lastname.map(UserLastName::new),
        };

        match self.controller.update_user(req) {
            Ok(_res) => {
                eprintln!("Successfully updated a user.")
            }
            Err(e) => {
                // TODO 丁寧なエラーハンドリング
                eprintln!("Failed to update a user: {:?}", e)
            }
        }
    }

    fn create_matches() -> AppArgs {
        /*

        let firstname_arg = Arg::new("firstname")
            .long("firstname")
            .short('f')
            .about("First name")
            .takes_value(true);
        let lastname_arg = Arg::new("lastname")
            .long("lastname")
            .short('l')
            .about("Last name")
            .takes_value(true);
        let email_arg = Arg::new("email")
            .long("email")
            .short('e')
            .about("Email address")
            .takes_value(true);

        App::new("User list")
            .version("1.0")
            .author("Sho Nakatani <lay.sakura@gmail.com>")
            .about("Example program to show how to use mockall crate.")
            .subcommand(
                App::new("search")
                    .about("Searches users by name and/or email address")
                    .arg(firstname_arg.clone())
                    .arg(lastname_arg.clone())
                    .arg(email_arg.clone()),
            )
            .subcommand(
                App::new("add")
                    .about("Adds a user")
                    .arg(firstname_arg.clone().required(true))
                    .arg(lastname_arg.clone().required(true))
                    .arg(email_arg.clone().required(true)),
            )
            .subcommand(
                App::new("update")
                    .about("Updates a user's name")
                    .arg(email_arg.clone().required(true))
                    .arg(firstname_arg.clone())
                    .arg(lastname_arg.clone()),
            )
            .get_matches()
            */
        AppArgs::parse()
    }
}
