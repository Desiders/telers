//! Combinations of the arguments and the variants of `#[derive(Command)]`, checked at runtime

use telers::{
    client::Reqwest,
    context::Context,
    filters::CommandObject,
    types::{ChatPrivate, MessageText, Update, UpdateMessage},
    utils::command_args::{Commands as _, Rest},
    Bot, Extensions, Extractor, Request,
};
use telers_macros::Command;

use std::{
    future::Future,
    pin::pin,
    sync::Arc,
    task::{Context as TaskContext, Poll, Waker},
};

// 1. Defaults: lowercase names, the `/` prefix and whitespace-separated arguments;
// unit, tuple and named variants
#[derive(Clone, Debug, PartialEq, Command)]
enum P01 {
    Start,
    Echo(String),
    Add { a: i64, b: i64 },
}

// 2. `snake_case` names with descriptions
#[derive(Clone, Debug, PartialEq, Command)]
#[command(rename_rule = "snake_case")]
enum P02 {
    #[command(description = "help")]
    HelpMe,
    #[command(description = "start")]
    Start,
}

// 3. Enum-level prefix with a variant-level override
#[derive(Clone, Debug, PartialEq, Command)]
#[command(prefix = '!')]
enum P03 {
    Start,
    #[command(prefix = '/')]
    Stop,
}

// 4. Enum-level split with a variant-level override and every kind of argument
#[derive(Clone, Debug, PartialEq, Command)]
#[command(split = ',')]
enum P04 {
    Add(i64, i64),
    #[command(split = ' ')]
    Echo(Rest),
    Maybe(Option<u8>),
    Many(Vec<u8>),
}

// 5. `rename`, `aliases` and `hidden`
#[derive(Clone, Debug, PartialEq, Command)]
enum P05 {
    #[command(rename = "go", aliases = ["begin", "run"], description = "start")]
    Start,
    #[command(hidden, description = "secret")]
    Secret,
    #[command(hidden, aliases = ["h"])]
    Hidden,
}

// 6. Everything at both levels with trailing commas
#[derive(Clone, Debug, PartialEq, Command)]
#[command(rename_rule = "snake_case", prefix = '!', split = ';',)]
enum P06 {
    #[command(description = "pair", prefix = '/', split = ',', rename = "pair-of", aliases = ["p"],)]
    PairOf(u8, u8),
    #[command(description = "sum")]
    SumAll(Vec<u8>),
    #[command(hidden)]
    SecretOne,
}

// 7. Enum without variants
#[derive(Clone, Debug, PartialEq, Command)]
enum P07 {}

fn request_with_command(command: Option<&str>) -> Request {
    let mut context = Context::new();
    if let Some(command) = command {
        context.insert("command", CommandObject::extract(command).unwrap());
    }

    Request {
        bot: Bot::default(),
        update: Arc::new(Update::Message(UpdateMessage::new(
            0,
            MessageText::new(0, 0, ChatPrivate::new(0), ""),
        ))),
        context,
        extensions: Extensions::default(),
    }
}

/// The extraction of the derived types is immediate, so one poll is enough
fn extract<T: Extractor<Reqwest>>(command: &str) -> Result<T, T::Error> {
    let request = request_with_command(Some(command));
    let mut fut = pin!(T::extract(&request));
    match fut.as_mut().poll(&mut TaskContext::from_waker(Waker::noop())) {
        Poll::Ready(res) => res,
        Poll::Pending => panic!("the extraction must be immediate"),
    }
}

fn bot_commands<T>(commands: Vec<telers::types::BotCommand>) -> Vec<(String, String)> {
    commands
        .into_iter()
        .map(|command| (command.command.into(), command.description.into()))
        .collect()
}

fn main() {
    // 1
    assert_eq!(extract::<P01>("/start").unwrap(), P01::Start);
    assert_eq!(extract::<P01>("/START").unwrap(), P01::Start);
    assert_eq!(
        extract::<P01>("/echo hello").unwrap(),
        P01::Echo("hello".to_owned())
    );
    assert_eq!(extract::<P01>("/add 1 2").unwrap(), P01::Add { a: 1, b: 2 });
    assert_eq!(P01Type::from(&P01::Add { a: 1, b: 2 }), P01Type::Add);
    assert_eq!(P01Type::from(&P01::Start), P01Type::Start);
    assert_eq!(P01::descriptions(), "/start\n/echo\n/add");
    assert_eq!(
        bot_commands::<P01>(P01::bot_commands()),
        [("start", ""), ("echo", ""), ("add", "")].map(|(command, description)| (command.to_owned(), description.to_owned()))
    );

    // 2
    assert_eq!(extract::<P02>("/help_me").unwrap(), P02::HelpMe);
    assert_eq!(P02::descriptions(), "/help_me - help\n/start - start");
    assert_eq!(
        bot_commands::<P02>(P02::bot_commands()),
        [("help_me", "help"), ("start", "start")].map(|(command, description)| (command.to_owned(), description.to_owned()))
    );

    // 3
    assert_eq!(extract::<P03>("!start").unwrap(), P03::Start);
    assert!(extract::<P03>("/start").is_err());
    assert_eq!(extract::<P03>("/stop").unwrap(), P03::Stop);
    assert!(extract::<P03>("!stop").is_err());
    assert_eq!(P03::descriptions(), "!start\n/stop");
    assert_eq!(
        bot_commands::<P03>(P03::bot_commands()),
        [("stop".to_owned(), String::new())]
    );

    // 4
    assert_eq!(extract::<P04>("/add 1,2").unwrap(), P04::Add(1, 2));
    assert_eq!(
        extract::<P04>("/echo a b c").unwrap(),
        P04::Echo(Rest("a b c".into()))
    );
    assert_eq!(extract::<P04>("/maybe").unwrap(), P04::Maybe(None));
    assert_eq!(extract::<P04>("/maybe 3").unwrap(), P04::Maybe(Some(3)));
    assert_eq!(extract::<P04>("/many 1,2,3").unwrap(), P04::Many(vec![1, 2, 3]));
    assert_eq!(extract::<P04>("/many").unwrap(), P04::Many(vec![]));

    // 5
    for command in ["/go", "/begin", "/run", "/GO"] {
        assert_eq!(extract::<P05>(command).unwrap(), P05::Start, "{command}");
    }
    assert!(extract::<P05>("/start").is_err());
    assert_eq!(extract::<P05>("/secret").unwrap(), P05::Secret);
    assert_eq!(extract::<P05>("/hidden").unwrap(), P05::Hidden);
    assert_eq!(extract::<P05>("/h").unwrap(), P05::Hidden);
    for name in ["go", "begin", "run"] {
        assert_eq!(P05::kind('/', name), Some(P05Type::Start), "{name}");
    }
    assert_eq!(P05::kind('/', "secret"), Some(P05Type::Secret));
    assert_eq!(P05::kind('/', "h"), Some(P05Type::Hidden));
    assert_eq!(P05::kind('/', "start"), None);
    assert_eq!(P05::kind('!', "go"), None);
    assert_eq!(P05Type::from(&P05::Secret), P05Type::Secret);
    assert_eq!(P05::descriptions(), "/go - start");
    assert_eq!(
        bot_commands::<P05>(P05::bot_commands()),
        [("go".to_owned(), "start".to_owned())]
    );

    // 6
    assert_eq!(extract::<P06>("/pair-of 1,2").unwrap(), P06::PairOf(1, 2));
    assert_eq!(extract::<P06>("/p 3,4").unwrap(), P06::PairOf(3, 4));
    assert!(extract::<P06>("!pair-of 1,2").is_err());
    assert_eq!(extract::<P06>("!sum_all 1;2;3").unwrap(), P06::SumAll(vec![1, 2, 3]));
    assert_eq!(extract::<P06>("!secret_one").unwrap(), P06::SecretOne);
    assert_eq!(P06::kind('/', "pair-of"), Some(P06Type::PairOf));
    assert_eq!(P06::kind('/', "p"), Some(P06Type::PairOf));
    assert_eq!(P06::kind('!', "sum_all"), Some(P06Type::SumAll));
    assert_eq!(P06::kind('!', "secret_one"), Some(P06Type::SecretOne));
    assert_eq!(P06::kind('!', "pair-of"), None);
    assert_eq!(P06Type::from(&P06::PairOf(1, 2)), P06Type::PairOf);
    assert_eq!(P06Type::from(&P06::SumAll(vec![])), P06Type::SumAll);
    assert_eq!(P06::descriptions(), "/pair-of - pair\n!sum_all - sum");
    assert_eq!(
        bot_commands::<P06>(P06::bot_commands()),
        [("pair-of".to_owned(), "pair".to_owned())]
    );

    // 7
    assert_eq!(P07::kind('/', "start"), None);
    assert_eq!(P07::descriptions(), "");
    assert!(P07::bot_commands().is_empty());
    assert!(extract::<P07>("/start").is_err());

    // 8. Parsing through the `Commands` trait, the same as the extraction
    let command = CommandObject::extract("/add 1,2").unwrap();
    assert_eq!(P04::parse(&command).unwrap(), P04::Add(1, 2));
    let command = CommandObject::extract("/add 1").unwrap();
    assert!(P04::parse(&command).is_err());
    let command = CommandObject::extract("/nothing").unwrap();
    assert!(P04::parse(&command).is_err());

    // 9. The parsed command kept in the context by the filter is extracted as is
    let mut context = Context::new();
    context.insert("parsed_command", P01::Echo("stored".to_owned()));
    let request: Request = Request {
        bot: Bot::default(),
        update: Arc::new(Update::Message(UpdateMessage::new(
            0,
            MessageText::new(0, 0, ChatPrivate::new(0), ""),
        ))),
        context,
        extensions: Extensions::default(),
    };
    let mut fut = pin!(P01::extract(&request));
    let Poll::Ready(res) = fut.as_mut().poll(&mut TaskContext::from_waker(Waker::noop())) else {
        panic!("the extraction must be immediate");
    };
    assert_eq!(res.unwrap(), P01::Echo("stored".to_owned()));

    // 10. Errors: unknown command, no command in the context, wrong arguments
    let err = extract::<P01>("/nothing").unwrap_err().to_string();
    assert!(err.contains("Unknown command `/nothing`"), "{err}");
    let request = request_with_command(None);
    let mut fut = pin!(P01::extract(&request));
    let Poll::Ready(res) = fut.as_mut().poll(&mut TaskContext::from_waker(Waker::noop())) else {
        panic!("the extraction must be immediate");
    };
    let err = res.unwrap_err().to_string();
    assert!(err.contains("`Command` filter"), "{err}");
    assert!(extract::<P01>("/add 1").is_err());
    assert!(extract::<P01>("/add 1 2 3").is_err());
    assert!(extract::<P01>("/add x 2").is_err());
}
