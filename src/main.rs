const OK: [char; 38] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.', '-', 'A', 'B', 'C', 'D', 'E', 'F', 'G',
    'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
const CONVERT_TABLE: [[&str; 2]; 27] = [
    ["A", "10"],
    ["B", "11"],
    ["C", "12"],
    ["D", "13"],
    ["E", "14"],
    ["F", "15"],
    ["G", "16"],
    ["H", "17"],
    ["I", "18"],
    ["J", "19"],
    ["K", "20"],
    ["L", "21"],
    ["M", "22"],
    ["N", "23"],
    ["O", "24"],
    ["P", "25"],
    ["Q", "26"],
    ["R", "27"],
    ["S", "28"],
    ["T", "29"],
    ["U", "30"],
    ["V", "31"],
    ["W", "32"],
    ["X", "33"],
    ["Y", "34"],
    ["Z", "35"],
    [".", "36"],
];
const COMMANDLINEARG: &str = r"
[USAGE]
main.exe --<mode> <option>
<mode> available in:
    error-code
    help
";
struct Information {
    number: String,
    minus: bool,
    float: bool,
    from: u8,
    to: u8,
}
impl Information {
    fn new(number: &str, minus: bool, float: bool, from: u8, to: u8) -> Self {
        Information {
            number: number.to_string(),
            minus,
            float,
            from,
            to,
        }
    }
    fn pretreatment(&mut self) {
        for character in self.number.as_str().chars() {
            if OK.iter().find(|&c| c == &character) == None {
                panic!("Function `pretreatment` failed. (Exit code: 0b10101010b10100110b10001010b10100100b1000000b10001010b10100100b10100100b10011110b10100100b1110100b1000000b11101100b11000010b11100100b11010010b11000010b11000100b11011000b11001010b1000000b11000000b11011100b11101010b11011010b11000100b11001010b11100100b11000000b1000000b11010010b11011100b11000110b11011000b11101010b11001000b11001010b11100110b1000000b11010010b11011100b11101100b11000010b11011000b11010010b11001000b1000000b11101100b11000010b11011000b11101010b11001010b11100110b100001)");
            }
        }
        if &self.number[..1] == "-" {
            self.minus = true;
            self.number.remove(0);
        }
        if self.from < 2 || 36 < self.from {
            panic!("Function `pretreatment` failed. (Exit code: 0b10101010b10100110b10001010b10100100b1000000b10001010b10100100b10100100b10011110b10100100b1110100b1000000b11101100b11000010b11011000b11010010b11000010b11000100b11011000b11001010b1000000b11000000b11001100b11100100b11011110b11011010b11000000b1000000b11011010b11101010b11100110b11101000b1000000b11000100b11001010b1000000b1100100b1000000b11011110b11100100b1000000b11011010b11011110b11100100b11001010b1000000b11000010b11011100b11001000b1000000b1100110b1101100b1000000b11011110b11100100b1000000b11011000b11001010b11100110b11100110b1000010b100000)");
        } else if self.to < 2 || 36 < self.to {
            panic!("Function `pretreatment` failed. (Exit code: 0b11101100b11000010b11011000b11010010b11000010b11000100b11011000b11001010b1000000b11000000b11101000b11011110b11000000b1000000b11011010b11101010b11100110b11101000b1000000b11000100b11001010b1000000b1100100b1000000b11011110b11100100b1000000b11011010b11011110b11100100b11001010b1000000b11000010b11011100b11001000b1000000b1100110b1101100b1000000b11011110b11100100b1000000b11011000b11001010b11100110b11100110b100001)");
        }
    }
    fn number_check(&mut self) {
        let mut number = alphabet2number(&self.number);
        if let Some(point) = number.iter().position(|x| *x == 36) {
            self.float = true;
            number.remove(point);
        }
        for num in number {
            if num >= self.from {
                panic!("Function `number_check` failed. (Exit code: 0b10101010b10100110b10001010b10100100b1000000b10001010b10100100b10100100b10011110b10100100b1110100b1000000b11011110b11011100b11001010b1000000b11011110b11001100b1000000b11101000b11010000b11001010b1000000b11101100b11000010b11100100b11010010b11000010b11000100b11011000b11001010b11100110b1000000b11000000b11011100b11101010b11011010b11000100b11001010b11100100b11000000b1000000b11001010b11110000b11000110b11001010b11001010b11001000b11100110b1000000b11101000b11010000b11001010b1000000b11011100b11101010b11011010b11001010b11100100b11010010b11000110b1000000b11100100b11000010b11011100b11001110b11001010b1000000b11011110b11001100b1000000b11101000b11010000b11001010b1000000b11001010b11011100b11101000b11001010b11100100b11001010b11001000b1000000b11011100b1011010b11000010b11100100b11110010b100001)");
            }
        }
    }
    fn integer_convert_to_decimal(&mut self) {
        let number = alphabet2number(&self.number);
        let mut count = number.len() as u32;
        let mut old = 0;
        let mut new = 0;
        for num in number {
            let converted = (self.from as u128).pow(count - 1) * num as u128;
            new = converted + old;
            old = new;
            count -= 1;
        }
        self.number = new.to_string();
    }
    fn float_convert_to_decimal(&mut self) {
        let mut number = alphabet2number(&self.number);
        #[allow(unused_assignments)]
        let mut count = 0i32;
        if let Some(point) = number.iter().position(|x| *x == 36) {
            number.remove(point);
            count = number[..point].len() as i32;
        } else {
            panic!("Function `float_convert_to_decimal` failed. (Exit code: 0b10100110b10110010b10100110b10101000b10001010b10011010b1000000b10001010b10100100b10100100b10011110b10100100b1110100b1000000b11001100b11011000b11011110b11000010b11101000b10111110b11000110b11011110b11011100b11101100b11001010b11100100b11101000b10111110b11101000b11011110b10111110b11001000b11001010b11000110b11010010b11011010b11000010b11011000b1110100b1110100b11001100b11000010b11010010b11011000b11001010b11001000b1000000b11101000b11011110b1000000b11001100b11010010b11011100b11001000b1000000b11001000b11001010b11000110b11010010b11011010b11000010b11011000b1000000b11100000b11011110b11010010b11011100b1110100)");
        }
        let mut old = 0.0;
        let mut new = 0.0;
        for num in number {
            let converted = (self.from as f64).powi(count - 1) * num as f64;
            new = converted + old;
            old = new;
            count -= 1;
        }
        self.number = new.to_string();
    }
    fn integer_calculate(&self) -> Vec<String> {
        let mut output = Vec::new();
        let mut number = self
            .number
            .parse::<u128>()
            .expect("Function `integer_calculate` failed. (Exit code: 0b10100110b10110010b10100110b10101000b10001010b10011010b1000000b10001010b10100100b10100100b10011110b10100100b1110100b1000000b11010010b11011100b11101000b11001010b11001110b11001010b11100100b10111110b11000110b11000010b11011000b11000110b11101010b11011000b11000010b11101000b11001010b1110100b1110100b11001010b11110000b11100000b11001010b11000110b1110100)");
        while number >= self.to as u128 {
            let division = number / self.to as u128;
            let modu = number % self.to as u128;
            if modu >= 10 {
                output.insert(0, number2alphabet(modu));
            } else {
                output.insert(0, modu.to_string());
            }
            number = division;
        }
        if number >= 10 {
            output.insert(0, number2alphabet(number));
        } else {
            output.insert(0, number.to_string());
        }
        if self.minus {
            output.insert(0, "-".to_string());
        }
        output
    }
    fn float_calculate(&mut self) -> Vec<String> {
        let mut output = vec!["0".to_string(), ".".to_string()];
        let number = self
            .number
            .parse::<f64>()
            .expect("Function `float_calculate` failed. (Exit code: 0b10100110b10110010b10100110b10101000b10001010b10011010b1000000b10001010b10100100b10100100b10011110b10100100b1110100b1000000b11001100b11011000b11011110b11000010b11101000b10111110b11000110b11000010b11011000b11000110b11101010b11011000b11000010b11101000b11001010b1110100b1110100b11001010b11110000b11100000b11001010b11000110b1110100)");
        let integer = number.floor();
        let mut decimal_point = number - integer;
        if integer >= 1.0 {
            self.number = (integer as u128).to_string();
            let integral = convert_vec_to_string(self.integer_calculate());
            output.remove(0);
            output.insert(0, integral);
        }
        while decimal_point != 0.0 {
            let decimal = decimal_point * self.to as f64;
            output.push((decimal as u64).to_string());
            decimal_point = decimal - decimal.floor();
        }
        if output[output.len() - 1] == ".".to_string() {
            output.remove(output.len() - 1);
        }
        output
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        loop {
            let number = input_upper("(`!exit` to exit) Which number to convert? >");
            if number == "!EXIT".to_string() {
                break;
            } else {
                let from = input_u8("The number is expressed in ");
                let to = input_u8("Make this number ");
                let mut instance = Information::new(&number, false, false, from, to);
                #[allow(unused_assignments)]
                let mut output = Vec::<String>::new();
                instance.pretreatment();
                instance.number_check();
                if instance.float {
                    instance.float_convert_to_decimal();
                    output = instance.float_calculate();
                } else {
                    instance.integer_convert_to_decimal();
                    output = instance.integer_calculate();
                }
                println!("Result -> {}", convert_vec_to_string(output));
            }
        }
    } else {
        commandline(args);
    }
}
fn commandline(args: Vec<String>) {
    match args.len() {
        2 => match args[1].as_str() {
            "--help" => println!(
                r"
    This program is capable of converting from binary to 36 base numbers.
    A detailed description about `error-code` can be found by the following means (This is all we have at the moment.):
        main.exe --help error-code
                    "
            ),
            _ => println!("{}", COMMANDLINEARG),
        },
        3 => match args[1].as_str() {
            "--error-code" => println!("{}", decode(&args[2])),
            "--help" => println!("{}", help(&args[2])),
            _ => println!("{}", COMMANDLINEARG),
        },
        _ => println!("{}", COMMANDLINEARG),
    }
}
fn input_upper(console: &str) -> String {
    println!("{} ", console);
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Function `input_upper` failed. (Exit code: 0b10100110b10110010b10100110b10101000b10001010b10011010b1000000b10001010b10100100b10100100b10011110b10100100b1110100b1000000b11010010b11011100b11100000b11101010b11101000b10111110b11101010b11100000b11100000b11001010b11100100b1110100b1110100b11001010b11110000b11100000b11001010b11000110b11101000b110001)");
    if input == "".to_string() {
        panic!("Function `input_upper` failed. (Exit code: 0b11101100b11000010b11100100b11010010b11000010b11000100b11011000b11001010b1000000b11000000b11010010b11011100b11100000b11101010b11101000b11000000b1000000b11010010b11100110b1000000b11000100b11011000b11000010b11011100b11010110b100001)")
    } else {
        input.trim().to_uppercase()
    }
}
fn input_u8(console: &str) -> u8 {
    println!("{}", console);
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Function `input_u8` failed. (Exit code: 0b10100110b10110010b10100110b10101000b10001010b10011010b1000000b10001010b10100100b10100100b10011110b10100100b1110100b1000000b11010010b11011100b11100000b11101010b11101000b10111110b11101010b1110000b1110100b1110100b11001010b11110000b11100000b11001010b11000110b11101000b110001)");
    if input == "".to_string() {
        panic!("Function `input_u8` failed. (Exit code: 0b11101100b11000010b11100100b11010010b11000010b11000100b11011000b11001010b1000000b11000000b11010010b11011100b11100000b11101010b11101000b11000000b1000000b11010010b11100110b1000000b11000100b11011000b11000010b11011100b11010110b100001)");
    } else {
        input
            .trim()
            .parse()
            .expect("Function `input_u8` failed. (Exit code: 0b10100110b10110010b10100110b10101000b10001010b10011010b1000000b10001010b10100100b10100100b10011110b10100100b1110100b1000000b11010010b11011100b11100000b11101010b11101000b10111110b11101010b1110000b1110100b1110100b11001010b11110000b11100000b11001010b11000110b11101000b110010)")
    }
}
fn alphabet2number(number: &str) -> Vec<u8> {
    let mut converted_number = Vec::new();
    for character in number.chars() {
        if character == '0'
            || character == '1'
            || character == '2'
            || character == '3'
            || character == '4'
            || character == '5'
            || character == '6'
            || character == '7'
            || character == '8'
            || character == '9'
        {
            converted_number.push(
                character
                    .to_string()
                    .parse()
                    .expect("Function `alphabet2number` failed. (Exit code: 0b10100110b10110010b10100110b10101000b10001010b10011010b1000000b10001010b10100100b10100100b10011110b10100100b1110100b1000000b11000010b11011000b11100000b11010000b11000010b11000100b11001010b11101000b1100100b11011100b11101010b11011010b11000100b11001010b11100100b1110100b1110100b11001010b11110000b11100000b11001010b11000110b11101000b110001)"),
            );
        } else {
            for alphabet in CONVERT_TABLE {
                if alphabet[0] == character.to_string().as_str() {
                    converted_number.push(
                        alphabet[1]
                            .parse()
                            .expect("Function `alphabet2number` failed. (Exit code: 0b10100110b10110010b10100110b10101000b10001010b10011010b1000000b10001010b10100100b10100100b10011110b10100100b1110100b1000000b11000010b11011000b11100000b11010000b11000010b11000100b11001010b11101000b1100100b11011100b11101010b11011010b11000100b11001010b11100100b1110100b1110100b11001010b11110000b11100000b11001010b11000110b11101000b110010)"),
                    );
                }
            }
        }
    }
    converted_number
}
fn number2alphabet(number: u128) -> String {
    if number == 0
        || number == 1
        || number == 2
        || number == 3
        || number == 4
        || number == 5
        || number == 6
        || number == 7
        || number == 8
        || number == 9
    {
        return number.to_string();
    } else {
        let mut string = String::new();
        for alphabet in CONVERT_TABLE {
            if alphabet[1] == number.to_string().as_str() {
                string = alphabet[0].to_string();
                break;
            }
        }
        if string == "".to_string() {
            panic!("Function `number2alphabet` failed. (Exit code: 0b10100110b10110010b10100110b10101000b10001010b10011010b1000000b10001010b10100100b10100100b10011110b10100100b1110100b1000000b11011100b11101010b11011010b11000100b11001010b11100100b1100100b11000010b11011000b11100000b11010000b11000010b11000100b11001010b11101000b1110100b1110100b11001010b11110000b11100000b11001010b11000110b1110100)");
        } else {
            return string;
        }
    }
}
fn convert_vec_to_string(vector: Vec<String>) -> String {
    let mut output = String::new();
    for character in vector.iter() {
        output += character;
    }
    output
}
fn decode(errorcode: &str) -> String {
    if errorcode.starts_with("0b") {
        let mut release = Vec::<String>::new();
        let mut raw = errorcode.split("0b").collect::<Vec<&str>>();
        raw.remove(0);
        for binary in raw {
            let mut instance = Information::new(binary, false, false, 2, 10);
            instance.integer_convert_to_decimal();
            let decimal = convert_vec_to_string(instance.integer_calculate());
            release.push(
                std::char::from_u32(
                    decimal
                        .parse()
                        .expect("Function `decode` failed. (Exit code: 0b10100110b10110010b10100110b10101000b10001010b10011010b1000000b10001010b10100100b10100100b10011110b10100100b1110100b1000000b11001000b11001010b11000110b11011110b11001000b11001010b1110100b1110100b11001010b11110000b11100000b11001010b11000110b11101000b110001)"),
                )
                .expect("Function `decode` failed. (Exit code: 0b10100110b10110010b10100110b10101000b10001010b10011010b1000000b10001010b10100100b10100100b10011110b10100100b1110100b1000000b11001000b11001010b11000110b11011110b11001000b11001010b1110100b1110100b11001010b11110000b11100000b11001010b11000110b11101000b110010)")
                .to_string()
            );
        }
        convert_vec_to_string(release)
    } else {
        panic!("Function `decode` failed. (Exit code: 0b10101010b10100110b10001010b10100100b1000000b10001010b10100100b10100100b10011110b10100100b1110100b1000000b11101100b11000010b11100100b11010010b11000010b11000100b11011000b11001010b1000000b11000000b11001010b11100100b11100100b11011110b11100100b11000110b11011110b11001000b11001010b11000000b1000000b11010010b11100110b1000000b11010010b11011100b11101100b11000010b11011000b11010010b11001000b1000000b11101100b11000010b11011000b11101010b11001010b100001)")
    }
}
fn help(function_name: &str) -> String {
    match function_name {
        "error-code" => return String::from(
                        r"
            [USAGE]
            main.exe --error-code <error code>
                            "
        ),
        _ => panic!("Function `help` failed. (Exit code: 0b10101010b10100110b10001010b10100100b1000000b10001010b10100100b10100100b10011110b10100100b1110100b1000000b11101100b11000010b11100100b11010010b11000010b11000100b11011000b11001010b1000000b11000000b11001100b11101010b11011100b11000110b11101000b11010010b11011110b11011100b10111110b11011100b11000010b11011010b11001010b11000000b1000000b11010010b11100110b1000000b11010010b11011100b11101100b11000010b11011000b11010010b11001000b1000000b11101100b11000010b11011000b11101010b11001010b100001)"),
    }
}
