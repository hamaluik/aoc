use anyhow::{Context, Result};
use chrono::TimeZone;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};

pub fn load_inputs() -> Result<()> {
    let inputs = check_missing_inputs();
    if inputs.is_empty() {
        return Ok(());
    }

    println!("The following inputs are missing:");
    for day in inputs.iter() {
        println!("    Day {}", day);
    }
    println!();

    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to download these inputs now?")
        .interact()
        .context("Failed to get confirmation")?;
    if !confirm {
        return Ok(());
    }

    let session = get_session().with_context(|| "Failed to get AOC session auth")?;
    inputs.into_iter().try_for_each(|day| {
        download_input(day, &session)?;
        println!("Downloaded day {day}!");
        Ok::<(), anyhow::Error>(())
    })?;

    Ok(())
}

fn check_missing_inputs() -> Vec<u32> {
    // timezone where AOC is hosted
    let tz = chrono::FixedOffset::west_opt(5 * 3600).expect("Valid timezone");

    let now = chrono::Utc::now();
    (1..=25)
        .filter(|day| {
            let date = tz
                .with_ymd_and_hms(2023, 12, *day, 0, 0, 1)
                .earliest()
                .expect("Valid date");
            now >= date
        })
        .filter(|day| {
            !std::path::Path::new("inputs")
                .join(&format!("day{:02}.txt", day))
                .exists()
        })
        .collect()
}

fn get_session() -> Result<String> {
    match std::env::var("AOC_SESSION") {
        Ok(session) => Ok(session),
        Err(_) => {
            println!("Inputs are missing and env var AOC_SESSION is not set!");
            println!("Please set AOC_SESSION to your AOC session cookie.");
            println!("You can find this by logging into AOC and looking at the value of the 'session' cookie.");
            println!(
                "It should look something like this: 53616c7465645f5f6a0a0a0a0a0a0a0a0a0a0a0a"
            );
            println!("You can set it by running:");
            println!("    export AOC_SESSION=53616c7465645f5f6a0a0a0a0a0a0a0a0a0a0a0a");
            println!("Or by adding it to a .env file in the root of this repo, like this:");
            println!("    echo 'AOC_SESSION=53616c7465645f5f6a0a0a0a0a0a0a0a0a0a0a0a' >> .env");
            println!("Be sure not to check this file into git! Add it to your .gitignore:");
            println!("    echo '.env' >> .gitignore");
            println!("Then run this command again.");

            println!("For now, you can just enter your session cookie here:");
            let session = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("AOC session cookie")
                .interact()
                .context("Failed to get session cookie")?;
            Ok(session)
        }
    }
}

fn download_input(day: u32, session: &str) -> Result<()> {
    let url = format!("https://adventofcode.com/2023/day/{day}/input");
    let body = ureq::get(&url)
        .set("Cookie", &format!("session={}", session))
        .call()
        .with_context(|| format!("Failed to communicate with AOC server for day {day}"))?
        .into_string()
        .with_context(|| format!("Failed to download input for day {}", day))?;
    let path = std::path::Path::new("inputs").join(&format!("day{:02}.txt", day));
    std::fs::write(&path, body).with_context(|| {
        format!(
            "Failed to write input for day {day} to file: {}",
            path.display()
        )
    })
}
