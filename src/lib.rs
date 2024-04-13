pub fn kilometers_to_miles(k: f32) -> f32 {
    k * 0.62
}

pub fn miles_to_kilometers(m: f32) -> f32 {
    m * 1.60
}

pub fn minutes_to_hours(minutes: i32) -> (i32, i32) {
    let hours_run = minutes / 60;
    let remaining_minutes = minutes % 60;
    println!("This is {hours_run} hours and {remaining_minutes} minutes");

    (hours_run, remaining_minutes)
}

//
// First full function of the program - getting per mile marathon time with hour and minute inputs
// I learned, while writing this function, that casting to an int or a float will round or --
// add decimal point. I did not know that before.

// This function is to achieve the per mile time of a marathon - with the goal time as input
// I know I can use some crate here like chrono to achieve better but this is a start
pub fn marathon_per_mile_time(hours: i32, minutes: i32) -> (i32, i32) {
    let total_seconds = (hours * 60 * 60) + minutes * 60;

    // Casting to an f32 here so I can divide by proper marathon distance
    let seconds_per_mile = total_seconds as f32 / 26.2;

    // Casting back to i32 so I can round and divide by 60 to get a proper per mile time
    let min_per_mile = seconds_per_mile as i32 / 60;

    // The seconds added onto the per minute per mile pace - for more precision
    // Casting again back to i32 - to round down
    let seconds_left_over = seconds_per_mile as i32 % 60;

    println!("To achieve this you would need to run {min_per_mile} minutes and {seconds_left_over} seconds per mile");
    (min_per_mile, seconds_left_over)
}
