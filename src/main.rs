fn main() {
    println!("My app to help runners!");
    let ometepe_race = 60;
    let native_ometepe = 100;
    let my_race = 10;
    let in_miles = kilometers_to_miles(my_race);
    let in_kilometers = miles_to_kilometers(ometepe_race);
    println!("My {}k is really {} miles :)", my_race, in_miles);
    println!(
        "My {} mile race is actually {} kilometers :)",
        ometepe_race, in_kilometers
    );
    println!(
        "But the island {}k on Ometepe is really {} miles :)",
        native_ometepe,
        kilometers_to_miles(native_ometepe)
    );
}

fn kilometers_to_miles(k: u8) -> f32 {
    k as f32 * 0.62
}

fn miles_to_kilometers(m: u8) -> f32 {
    m as f32 * 1.60
}
