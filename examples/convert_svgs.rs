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

fn get_pct(x: i32, divisor: i32) -> i32 {
    (x * 100) / divisor
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

fn btn_transforms() {
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
        let file = format!("svgs/{btn}_btn.svg");
        let (w, h) = identify_size(&file);

        let pct = get_pct(BTN_HEIGHT, h);
        let w = pct_of(pct, w);

        let cmd = format!("svgexport svgs/{btn}_btn.svg {OUT}/{btn}_btn.svg {w}:{h}");
        exec(&cmd);
    }
}

fn texts() {
    let texts = ["victory", "defeat", "paused", "level"];

    for (i, text) in texts.iter().enumerate() {
        let file = format!("svgs/{text}.svg");
        let (w, h) = identify_size(&file);
        let th = if i == 3 { LEVEL_HEIGHT } else { TEXT_HEIGHT };

        let pct = get_pct(th, h);
        let w = pct_of(pct, w);

        let cmd = format!("svgexport svgs/{text}.svg {OUT}/{text}.png {w}:{h}");
        exec(&cmd);
    }
}

fn kodamas() {
    let colors = ["red", "green", "blue", "yellow", "orange"];

    for color in colors {
        let cmd = format!(
            "svgexport svgs/{color}_kodama.svg {OUT}/{color}_kodama.png {DOT_SIZE}:{DOT_SIZE}"
        );
        exec(&cmd);

        let cmd =
            format!("svgexport svgs/{color}_dot.svg {OUT}/{color}_dot.png {DOT_SIZE}:{DOT_SIZE}");
        exec(&cmd);
    }

    let cmd = format!(
        "svgexport svgs/connector.svg {OUT}/connector.png {CONNECTOR_SIZE}:{CONNECTOR_SIZE}"
    );
    exec(&cmd);
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
        let file = format!("svgs/{btn}_btn.svg");
        let (w, h) = identify_size(&file);
        let cmd = format!("svgexport {file} {OUT}/{btn}_btn.png {w}:{h}");
        exec(&cmd);
    }
}

fn numbers() {
    for n in 1..21 {
        let file = format!("svgs/{}.svg", 1);
        let (w, h) = identify_size(&file);
        let name = number_to_str(n);
        let outfile = format!("{OUT}/{name}.png");
        let cmd = format!("svgexport svgs/{n}.svg {outfile} {w}:{h}");
        exec(&cmd);
    }
}

fn main() {
    std::fs::remove_dir_all(OUT).expect("rm failed");
    std::fs::create_dir(OUT).expect("mkdir failed");

    kodamas();
    btn_transforms();
    texts();
    button_transforms();
    numbers();
}

fn exec(cmd: &String) -> String {
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
