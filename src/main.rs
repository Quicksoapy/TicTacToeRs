use std::io::stdin;

fn main() {
    let mut finish: bool = false;
    //Creates a 3*3 8-bit integer two-dimensional array
    let mut values = [[' ';3]; 3];
    let mut input_x = String::new();
    let mut input_y = String::new();

    println!("Hi! This is soapy's TicTacToe, an exercise she used to learn Rust with.");
    println!("To choose a place, type coordinates, from 0 0 to 2 2.");
    print_field(values);

    while finish == false {
        input_x.clear();
        input_y.clear();
        stdin().read_line(&mut input_x).expect("Failed to read line.");
        let input_value_x = match input_x.trim().parse() {
            Ok(num @ 0..=2) => num,
            _ => {
                println!("False input. Valid inputs: 1 2 3 4 5 6 7 8 9.");
                println!("Enter horizontal coordinate: ");
                continue;
            },
        };

        stdin().read_line(&mut input_y).expect("Failed to read line.");
        let input_value_y = match input_y.trim().parse() {
            Ok(num @ 0..=2) => num,
            _ => {
                println!("False input. Valid inputs: 1 2 3 4 5 6 7 8 9.");
                println!("Enter horizontal coordinate: ");
                continue;
            },
        };

        if values[input_value_x][input_value_y] != ' '{
            println!("That field already has a symbol in it. Choose something else.");
            println!("Enter horizontal coordinate: ");
            continue;
        }
        values[input_value_x][input_value_y] = 'x';
        print_field(values);
        finish = check_if_finished(values);
    }

}

fn print_field(values_array: [[char; 3]; 3]) {
    println!("-------------\n\
              - {} - {} - {} -\n\
              -------------\n\
              - {} - {} - {} -\n\
              -------------\n\
              - {} - {} - {} -\n\
              -------------", values_array[0][0], values_array[0][1], values_array[0][2], values_array[1][0],
             values_array[1][1], values_array[1][2], values_array[2][0], values_array[2][1], values_array[2][2])
}

fn check_if_finished(values_array: [[char; 3]; 3]) -> bool {
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
    if !values_array.contains(&[' ';3]){
        println!("Finished!");
        return true;
    }
    return false;
}

/*
fn bot_location_pick(values_array: [char; 9]) -> u8 {

    return 1;
}
*/