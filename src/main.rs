use std::io::stdin;

fn main() {
    let mut finish: bool = false;
    //Creates a 3*3 8-bit integer two-dimensional array
    let mut values = [[0u8;2]; 2];
    let mut input = String::new();

    println!("Hi! This is soapy's TicTacToe, an exercise she used to learn Rust with.");
    println!("To choose a place, type 1-9, 1 for top left, 2 for middle left, 9 for bottom right.");
    print_field(values);

    while finish == false {
        input.clear();
        stdin().read_line(&mut input).expect("Failed to read line.");

        let input_value = match input.trim().parse() {
            Ok(num @ 1..=9) => num,
            _ => {
                println!("False input. Valid inputs: 1 2 3 4 5 6 7 8 9");
                continue;
            },
        };
        if input_value < 1 || input_value > 9 {
            println!("False input. Valid inputs: 1 2 3 4 5 6 7 8 9");
            continue;
        }
        if values[input_value - 1] != ' '{
            println!("That field already has a symbol in it. Choose something else.");
            continue;
        }
        values[input_value - 1] = 'x';
        print_field(values);
        finish = check_if_finished(values);
    }

}

fn print_field(values_array: [char; 9]) {
    println!("-------------\n\
              - {} - {} - {} -\n\
              -------------\n\
              - {} - {} - {} -\n\
              -------------\n\
              - {} - {} - {} -\n\
              -------------", values_array[0], values_array[1], values_array[2], values_array[3],
             values_array[4], values_array[5], values_array[6], values_array[7], values_array[8])
}

fn check_if_finished(values_array: [char; 9]) -> bool {
    /*
    1 2 3
    1 5 9
    1 4 7
    2 5 8
    3 5 7
    3 6 9
    4 5 6
    7 8 9
     */
    if !values_array.contains(&' '){
        println!("Finished!");
        return true;
    }
    return false;
}

fn bot_location_pick(values_array: [char; 9]) -> u8 {

    return 1;
}
