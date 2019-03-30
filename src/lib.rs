use regex::Regex;
use std::error::Error;
use std::fmt;
use std::net::Ipv4Addr;
use std::process::Command;

#[derive(PartialEq, Clone, Debug)]
pub struct DefaultRoute {
    gateway: Ipv4Addr,
    dev: String,
}

impl DefaultRoute {
    pub fn new(gateway: Ipv4Addr, dev: String) -> DefaultRoute {
        DefaultRoute { gateway, dev }
    }

    pub fn gateway(&self) -> Ipv4Addr {
        self.gateway.clone()
    }

    pub fn dev(&self) -> String {
        self.dev.clone()
    }
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

pub fn get_default_routes() -> Result<Vec<DefaultRoute>, Box<dyn Error>> {
    let output = Command::new("ip").arg("route").arg("list").output()?;

    if !output.status.success() {
        return Err(Box::new(RunError {}));
    }

    let pattern = Regex::new(r"(default\svia\s)(.*)(\sdev\s)([0-9a-zA-Z]*)(\s|\n)")?;

    let routes = String::from_utf8(output.stdout)?
        .lines()
        .filter_map(move |line| pattern.captures(line))
        .map(|cap| DefaultRoute::new(cap[2].parse::<Ipv4Addr>().unwrap(), cap[4].to_string()))
        .collect();

    Ok(routes)
}

pub fn del_default_route(route: DefaultRoute) -> Result<(), Box<dyn Error>> {
    let output = Command::new("ip")
        .arg("route")
        .arg("del")
        .arg("default")
        .arg("via")
        .arg(route.gateway().to_string())
        .arg("dev")
        .arg(route.dev())
        .output()?;

    if !output.status.success() {
        return Err(Box::new(RunError {}));
    }

    Ok(())
}

pub fn add_default_route(route: DefaultRoute) -> Result<(), Box<dyn Error>> {
    let output = Command::new("ip")
        .arg("route")
        .arg("add")
        .arg("default")
        .arg("via")
        .arg(route.gateway().to_string())
        .arg("dev")
        .arg(route.dev())
        .output()?;

    if !output.status.success() {
        return Err(Box::new(RunError {}));
    }

    Ok(())
}
