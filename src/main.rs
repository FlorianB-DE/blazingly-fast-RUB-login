use std::{thread, time::Duration, collections::HashMap};

use confy::load;
use local_ip_address::list_afinet_netifas;
use serde_derive::{Deserialize, Serialize};

static CONFIG_NAME: &'static str = "config";

fn main() {
    let config = get_config();

    if config.login_id.is_empty() || config.password.is_empty() {
        panic!("No login id or password provided")
    }

    loop {
        println!("testing internet access");
        if !has_internet_access(&config.remote_target) {
            let address = get_ip_address(&config);

            if post_credentials(&config, address) {
                println!("Authetication Successfull")
            }
            else {
                eprintln!("Authetication Failed. Retrying in 10 min")
            }
        }

        else {
            println!("internet works, see you in 10 mins");
        }
        thread::sleep(Duration::from_secs(60 * 10)); // try again in 10 mins
    }
}

fn get_ip_address(config: &Config) -> String {
    let network_interfaces =
        list_afinet_netifas().expect("Could not load network interfaces: Aborting...");

    network_interfaces
        .iter()
        .find(|e| {
            let (_, address) = e;
            address.to_string().starts_with(&config.ip_prefix)
        })
        .expect("Could not find interface with given IP restraints")
        .1
        .to_string()
}

fn has_internet_access(to: &String) -> bool {
    let client = reqwest::blocking::Client::new();
    let res = match client.post(to).send() {
        Ok(r) => r,
        Err(_) => return false,
    };

    res.status().is_success()
}

fn post_credentials(config: &Config, ip_address: String) -> bool {
    let client = reqwest::blocking::Client::new();
    let mut params: HashMap<&str, &str> = HashMap::new();
    params.insert("code", "1");
    params.insert("loginid", &config.login_id);
    params.insert("password", &config.password);
    params.insert("ipaddr", &ip_address);
    params.insert("action", "Login");

    println!("trying to log in to {} as {} with address: {ip_address}", &config.login_address, &config.login_id);

    let res = match client.post(&config.login_address)
    .form(&params)
    .send() {
        Ok(r) => r,
        Err(_) => return false,
    };

    res.status().is_success() && res.text().unwrap_or_default().contains("Authentisierung gelungen")
}

#[inline]
fn get_config() -> Config {
    load::<Config>(env!("CARGO_PKG_NAME"), CONFIG_NAME)
        .expect("Could not load or create config file! Aborting...")
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    ip_prefix: String,
    login_id: String,
    password: String,
    remote_target: String,
    login_address: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ip_prefix: String::from("10."),
            login_id: String::new(),
            password: String::new(),
            remote_target: String::from("https://ismyinternetworking.com/whatsmyinfo"),
            login_address: String::from("https://login.ruhr-uni-bochum.de/cgi-bin/laklogin")
        }
    }
}
