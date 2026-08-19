use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::num::ParseIntError;
use thiserror::Error;

// #[derive(Debug, Error)]
// enum ParseError {
//     #[error("wrong number of tokens {0} expected 2" )]
//     WrongNumberOfTokens(i32),
//     #[error("We couldn't parse an integer: {0}")]
//     InvalidInt(#[from] ParseIntError),
// }

#[derive(Debug)]
enum ParseError {
    WrongNumberOfTokens(usize),
    InvalidInt(String),
}

// Player types: "goblin 10 20"  →  spawn a goblin at (10, 20)
#[derive(Debug, Eq, PartialEq)]
struct SpawnCommand {
    npc_type: String,
    x: i32,
    y: i32,
}

// Step 1: fully manual — matches, translates by hand
fn parse_spawn(input: &str) -> Result<SpawnCommand, ParseError> {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.len() != 3 {
        return Err(ParseError::WrongNumberOfTokens(tokens.len()));
    }

    let x = match tokens[1].parse::<i32>() {
        Ok(x) => x,
        Err(e) => return Err(ParseError::InvalidInt(e.to_string())),
    };
    let y = match tokens[2].parse::<i32>() {
        Ok(y) => y,
        Err(e) => return Err(ParseError::InvalidInt(e.to_string())),
    };

    Ok(SpawnCommand { npc_type: tokens[0].to_string(), x, y })
}

// Step 2: implement From, then let ? do the translation
impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError::InvalidInt(e.to_string())
    }
}

fn parse_spawn_from(input: &str) -> Result<SpawnCommand, ParseError> {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.len() != 3 {
        return Err(ParseError::WrongNumberOfTokens(tokens.len()));
    }

    let x = tokens[1].parse::<i32>()?;
    let y = tokens[2].parse::<i32>()?;

    Ok(SpawnCommand { npc_type: tokens[0].to_string(), x, y })
}

// Step 3: Our errors should be at the right level of abstraction.  Fix that WrongNumberOfTokens
// #[derive(Debug)]
// enum ParseErrorClean {
//     MalformedCommand(String),
//     InvalidCoordinate(ParseIntError),
// }
//
// impl Display for ParseErrorClean {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             ParseErrorClean::MalformedCommand(msg) =>
//                 write!(f, "invalid command format '{}': expected '<type> <x> <y>'", msg),
//             ParseErrorClean::InvalidCoordinate(e) =>
//                 write!(f, "failed to parse coordinate as an integer: {}", e),
//         }
//     }
// }
//
// impl Error for ParseErrorClean {}


// Step 4: automate Error, Display, and From with thiserror
#[derive(Error, Debug)]
enum ParseErrorClean {
    #[error("invalid command format '{0}': expected '<type> <x> <y>'")]
    MalformedCommand(String),
    #[error("failed to parse coordinate as an integer")]
    InvalidCoordinate(#[from]ParseIntError),
}

fn parse_spawn_clean(input: &str) -> Result<SpawnCommand, ParseErrorClean> {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.len() != 3 {
        return Err(ParseErrorClean::MalformedCommand(input.to_string()));
    }

    let x = tokens[1].parse::<i32>()?;
    let y = tokens[2].parse::<i32>()?;

    Ok(SpawnCommand { npc_type: tokens[0].to_string(), x, y })
}


fn parse_no(s: &str) {
    let x = s.parse::<u32>();
    match x {
        Ok(x) => println!("spawning at x = {x}"),
        Err(e) => println!("bad coordinate: {e}"),
    }
}

// Step 3: propagating manually
fn parse_coord(s: &str) -> Result<u32, ParseIntError> {
    let x = match s.parse::<u32>() {
        Ok(good) => good,
        Err(e) => return Err(e),
    };
    println!("just read {x}");
    Ok(x)
}

// Step 4: When is unwrap appropriate?
fn parse_coord_unwrap(s: &str) -> u32 {
    let x = s.parse::<u32>().unwrap();
    println!("just read {x}");
    x
}




fn main() {
    // let result = add_2("32 16x");
    // match add_2("32 16x") {
    //     Ok(sum) => println!("Sum: {sum}"),
    //     Err(err) => println!("Error: {err}"),
    // }

}


#[cfg(test)]
mod tests {
    #![allow(clippy::panic, clippy::unwrap_used)]
    use super::*;

    #[test]
    fn test_parse_0() {
        let x:u32 = get_string_from_file().parse().unwrap();
        assert_eq!(x, 32);
    }

    fn get_string_from_file() -> String {
        "42".to_string()
    }

    #[test]
    fn parse_spawn_valid() {
        let cmd = parse_spawn("goblin 10 20").unwrap();
        assert_eq!(cmd, SpawnCommand { npc_type: "goblin".into(), x: 10, y: 20 });
    }

    #[test]
    fn parse_spawn_wrong_token_count() {
        let err = parse_spawn("goblin 10");
        assert!(matches!(err, Err(ParseError::WrongNumberOfTokens(2))));
    }

    #[test]
    fn parse_spawn_bad_x() {
        let err = parse_spawn("goblin ten 20");
        assert!(matches!(err, Err(ParseError::InvalidInt(_))));
    }

    #[test]
    fn parse_spawn_bad_y() {
        let err = parse_spawn("goblin 10 twenty");
        assert!(matches!(err, Err(ParseError::InvalidInt(_))));
    }

    // --- parse_spawn_from (Step 2: From + ?) ---

    #[test]
    fn parse_spawn_from_valid() {
        let cmd = parse_spawn_from("skeleton 5 -3").unwrap();
        assert_eq!(cmd, SpawnCommand { npc_type: "skeleton".into(), x: 5, y: -3 });

    }

    #[test]
    fn parse_spawn_from_wrong_token_count() {
        let err = parse_spawn_from("skeleton 5 -3 extra");
        assert!(matches!(err, Err(ParseError::WrongNumberOfTokens(4))));
    }

    #[test]
    fn parse_int_error_maps_to_invalid_coordinate() {
        let parse_err: ParseIntError = "abc".parse::<i32>().unwrap_err();
        let mapped = ParseError::from(parse_err.clone());

        assert!(matches!(mapped, ParseError::InvalidInt(_)));
    }

    #[test]
    fn parse_spawn_from_bad_coordinate() {
        let err = parse_spawn_from("skeleton five -3");
        assert!(matches!(err, Err(ParseError::InvalidInt(_))));
    }

    // --- parse_spawn_clean (Step 3: error at the right abstraction level) ---

    #[test]
    fn parse_spawn_clean_valid() {
        let cmd = parse_spawn_clean("orc 1 2").unwrap();
        assert_eq!(cmd, SpawnCommand { npc_type: "orc".to_string(), x: 1, y: 2 });

    }

    #[test]
    fn parse_spawn_clean_malformed_keeps_original_input() {
        let err = parse_spawn_clean("orc 1");
        match err {
            Err(ParseErrorClean::MalformedCommand(original)) => assert_eq!(original, "orc 1"),
            other => panic!("expected MalformedCommand, got {other:?}"),
        }
    }

    #[test]
    fn parse_spawn_clean_bad_coordinate() {
        let err = parse_spawn_clean("orc one 2");
        assert!(matches!(err, Err(ParseErrorClean::InvalidCoordinate(_))));
    }

    #[test]
    fn parse_int_error_maps_to_invalid_coordinate_clean() {
        let parse_err: ParseIntError = "abc".parse::<i32>().unwrap_err();
        let mapped = ParseErrorClean::from(parse_err.clone());

        assert!(matches!(mapped, ParseErrorClean::InvalidCoordinate(parse_err)));
    }
    // --- confirms the From impl actually fires, not just compiles ---

    #[test]
    fn parse_int_error_converts_via_from() {
        let res = "abc".parse::<i32>();
        match res {
            Ok(_) => panic!("expected ParseIntError, got Ok"),
            Err(e) => { let converted: ParseErrorClean = e.into();
                assert!(matches!(converted, ParseErrorClean::InvalidCoordinate(_)));}
        }

    }

    #[test]
    fn malformed_command_displays_clear_user_message() {
        let err = ParseErrorClean::MalformedCommand("goblin 10".to_string());
        // .to_string() tests the Display implementation
        assert_eq!(
            err.to_string(),
            "invalid command format 'goblin 10': expected '<type> <x> <y>'"
        );
    }

    #[test]
    fn invalid_coordinate_displays_clear_user_message() {
        let err = ParseErrorClean::InvalidCoordinate("abc".parse::<i32>().unwrap_err());
        // .to_string() tests the Display implementation
        assert_eq!(
            err.to_string(),
            "failed to parse coordinate as an integer"
        );
    }
}