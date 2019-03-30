use regex::Regex;
use std::process::Command;
use std::error::Error;
use std::fmt;

#[derive(PartialEq, Default, Clone, Debug)]
struct Commit {
    hash: String,
    message: String,
}

#[derive(Debug)]
struct RunError;

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "run command failed")
    }
}

impl Error for RunError {
    fn description(&self) -> &str {
        "run command failed"
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let output = Command::new("git").arg("log").arg("--oneline").output()?;

    if !output.status.success() {
        return Err(Box::new(RunError {}));
    }

    let pattern = Regex::new(r"(?x)
                                ([0-9a-fA-f]+) # commit hash 
                                (.*)           # The commit message")?;
    
    String::from_utf8(output.stdout)?
        .lines()
        .filter_map(|line| pattern.captures(line))
        .map(|cap| {
            Commit {
                hash: cap[1].to_string(),
                message: cap[2].trim().to_string(),
            }
        })
        .take(5)
        .for_each(|x| println!("{:?}", x));

    Ok(())
}