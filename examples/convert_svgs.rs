use std::process::Command;

const OUT: &str = "outvgs";
const DOT_SIZE: i32 = 160;

const CONNECTOR_SIZE: i32 = pct_of(28, DOT_SIZE);

const LEVEL_HEIGHT: i32 = pct_of(70, DOT_SIZE);
const TEXT_HEIGHT: i32 = pct_of(85, DOT_SIZE * 2);
const BTN_HEIGHT: i32 = pct_of(80, DOT_SIZE * 2);

const fn pct_of(pct: i32, n: i32) -> i32 {
    (pct * n) / 100
}

fn get_pct(n: i32, divisor: i32) -> i32 {
    (n * 100) / divisor
}

fn number_to_str(n: i32) -> String {
    let n = match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        20 => "twenty",
        _ => panic!("{n} shouldnt be passed here"),
    };

    String::from(n)
}

fn identify_size(file: &String) -> (i32, i32) {
    let cmd = format!("identify {file}");
    let resp = exec(&cmd);
    let resp = resp.split(' ');

    let dims = if let Some(dims) = resp.skip(2).next() {
        dims
    } else {
        panic!("tf?")
    };

    let mut dims = dims.split('x');

    let w = if let Some(w) = dims.next() {
        w.parse::<i32>().unwrap()
    } else {
        panic!("tfw")
    };

    let h = if let Some(h) = dims.next() {
        h.parse::<i32>().unwrap()
    } else {
        panic!("tfh")
    };

    (w, h)
}

fn get_resize(file: &String, height: i32) -> (i32, i32) {
    let (w, h) = identify_size(&file);

    let pct = get_pct(height, h);
    let w = pct_of(pct, w);

    (w, h)
}

fn reverse_resize(file: &String, height: i32) -> (i32, i32) {
    let (w, h) = identify_size(&file);

    let pct = get_pct(h, height);
    let w = pct_of(pct, w);

    (w, h)
}

fn convert(name: &String, w: i32, h: i32) {
    // let cmd = format!("svgexport svgs/{name}.svg {OUT}/{name}.png {w}:{h}");
    let cmd = format!("inkscape --export-filename={OUT}/{name}.png -w {w} -h {h} svgs/{name}.svg");
    exec(&cmd);
}

fn convert_dest(name: &String, dest: &String, w: i32, h: i32) {
    // let cmd = format!("svgexport svgs/{name}.svg {OUT}/{dest}.png {w}:{h}");
    let cmd = format!("inkscape --export-filename={OUT}/{dest}.png -w {w} -h {h} svgs/{name}.svg");
    exec(&cmd);
}

fn texts() {
    let texts = ["victory", "defeat", "paused", "level"];

    for (i, text) in texts.iter().enumerate() {
        let file = format!("svgs/{text}.svg");
        let th = if i == 3 { LEVEL_HEIGHT } else { TEXT_HEIGHT };

        let (w, h) = get_resize(&file, th);

        let name = format!("{}", text);
        convert(&name, w, h);
    }
}

fn kodamas() {
    let colors = ["red", "green", "blue", "yellow", "orange"];

    let (w, h) = (DOT_SIZE, DOT_SIZE);
    for color in colors {
        let name = format!("{color}_kodama");
        convert(&name, w, h);

        let name = format!("{color}_dot");
        convert(&name, w, h);
    }

    let name = String::from("connector");
    let (w, h) = (CONNECTOR_SIZE, CONNECTOR_SIZE);
    convert(&name, w, h);
}

fn button_transforms() {
    let btns = [
        "menu",
        "new_game",
        "minus",
        "plus",
        "resume",
        "quit",
        "play",
        "about",
        "menu_light",
        "next_level",
    ];

    for btn in btns {
        let name = format!("{btn}_btn");
        let file = format!("svgs/{name}.svg");
        let (w, _h) = get_resize(&file, BTN_HEIGHT);

        convert(&name, w, BTN_HEIGHT);
    }
}

fn numbers() {
    for n in 1..21 {
        let name = format!("{n}");
        let dest = format!("{}", number_to_str(n));
        let file = format!("svgs/{name}.svg");
        let (w, h) = reverse_resize(&file, TEXT_HEIGHT);

        convert_dest(&name, &dest, w, h);
    }
}

fn main() {
    let result = std::fs::remove_dir_all(OUT);
    match result {
        Ok(_) => {
            std::fs::create_dir(OUT).expect("mkdir failed");
        }
        Err(_) => {
            std::fs::create_dir(OUT).expect("mkdir failed");
        }
    }

    kodamas();
    numbers();
    texts();
    button_transforms();
}

fn exec(cmd: &String) -> String {
    println!("{}", cmd);
    let mut cmds = cmd.split(' ');
    if let Some(program) = cmds.next() {
        let mut cmd = Command::new(program);

        loop {
            if let Some(arg) = cmds.next() {
                cmd.arg(arg);
            } else {
                break;
            }
        }

        let output = cmd.output().expect("cmd should pass");

        let stderr = output.stderr;

        if !stderr.is_empty() {
            let s = match std::str::from_utf8(&stderr) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };

            panic!("{s}");
        }

        let stdout = match std::str::from_utf8(&output.stdout) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        String::from(stdout)
    } else {
        panic!("invalid command: {cmd}")
    }
}
