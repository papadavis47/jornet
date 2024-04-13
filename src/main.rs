fn main() {
    println!("My app to help runners!");
    let ometepe_race = 60 as f32;
    let native_ometepe = 100 as f32;
    let my_race = 10 as f32;
    let in_miles = jornet::kilometers_to_miles(my_race);
    let in_kilometers = jornet::miles_to_kilometers(ometepe_race);
    println!("My {}k is really {} miles :)", my_race, in_miles);
    println!(
        "My {} mile race is actually {} kilometers :)",
        ometepe_race, in_kilometers
    );
    println!(
        "But the island {}k on Ometepe is really {} miles :)",
        native_ometepe,
        jornet::kilometers_to_miles(native_ometepe)
    );

    let testing_function_minutes_to_hours = jornet::minutes_to_hours(330);
    println!("{:?}", testing_function_minutes_to_hours);

    let testing_marathon_calc = jornet::marathon_per_mile_time(4, 15);
    println!("{:?}", testing_marathon_calc);
    let another_number_test = 11 / 4;
    println!("Another number test: {}", another_number_test)
}
