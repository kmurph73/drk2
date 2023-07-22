const out = "outvgs";
const dotSize = 160;
const connectorSize = Math.floor(dotSize * 0.28);

const colors = ["red", "green", "blue", "yellow", "orange"];

const levelHeight = dotSize * 0.7;
const textHeight = dotSize * 2 * 0.85;
const btnHeight = dotSize * 2 * 0.8;

const numberMap: Record<string, string> = {
  1: "one",
  2: "two",
  3: "three",
  4: "four",
  5: "five",
  6: "six",
  7: "seven",
  8: "eight",
  9: "nine",
  10: "ten",
  11: "eleven",
  12: "twelve",
  13: "thirteen",
  14: "fourteen",
  15: "fifteen",
  16: "sixteen",
  17: "seventeen",
  18: "eighteen",
  19: "nineteen",
  20: "twenty",
};

const buttonTransforms = async () => {
  let commands = [];

  const btns = [
    "menu",
    "new_game",
    "minus",
    "plus",
    "resume",
    "quit",
    "play",
    "about",
    "menu_light",
  ];

  for (let index = 0; index < btns.length; index++) {
    const btn = btns[index];

    const command = new Deno.Command("identify", {
      args: [`svgs/${btn}_btn.svg`],
    });

    commands.push(command.output());
  }

  const responses = await Promise.all(commands);

  commands = [];

  for (let index = 0; index < responses.length; index++) {
    const { stdout } = responses[index];
    const resp = new TextDecoder().decode(stdout);

    const [_name, _t, dims] = resp.split(" ");
    let [w, h] = dims.split("x").map((s) => parseInt(s));

    const ratio = btnHeight / h;

    w = Math.floor(ratio * w);

    const btn = btns[index];

    const command = new Deno.Command("inkscape", {
      args: [
        `--export-filename=${out}/${btn}_btn.png`,
        "-w",
        w.toString(),
        "-h",
        btnHeight.toString(),
        `svgs/${btn}_btn.svg`,
      ],
    });

    commands.push(command.output());
  }

  await Promise.all(commands);
};

const texts = async () => {
  let commands = [];

  const texts = ["victory", "defeat", "paused", "level"];

  for (let index = 0; index < texts.length; index++) {
    const text = texts[index];

    const command = new Deno.Command("identify", {
      args: [`svgs/${text}.svg`],
    });

    commands.push(command.output());
  }

  const responses = await Promise.all(commands);

  commands = [];

  for (let index = 0; index < responses.length; index++) {
    const { stdout } = responses[index];
    const resp = new TextDecoder().decode(stdout);

    const [_name, _t, dims] = resp.split(" ");
    let [w, h] = dims.split("x").map((s) => parseInt(s));

    const th = texts[index] === "level" ? levelHeight : textHeight;
    const ratio = th / h;

    w = Math.floor(ratio * w);

    const text = texts[index];

    const command = new Deno.Command("inkscape", {
      args: [
        `--export-filename=${out}/${text}.png`,
        "-w",
        w.toString(),
        "-h",
        th.toString(),
        `svgs/${text}.svg`,
      ],
    });

    commands.push(command.output());
  }

  await Promise.all(commands);
};

const numbers = async () => {
  let commands = [];

  for (let index = 0; index < 20; index++) {
    const command = new Deno.Command("identify", {
      args: [`svgs/${index + 1}.svg`],
    });

    commands.push(command.output());
  }

  const responses = await Promise.all(commands);

  commands = [];
  for (let index = 0; index < 20; index++) {
    const { stdout } = responses[index];
    const resp = new TextDecoder().decode(stdout);

    const [_name, _t, dims] = resp.split(" ");
    let [w, h] = dims.split("x").map((s) => parseInt(s));

    const ratio = h / textHeight;

    w = Math.floor(ratio * w);

    const name = `${index + 1}`;
    const command = new Deno.Command("inkscape", {
      args: [
        `--export-filename=${out}/${numberMap[name]}.png`,
        "-w",
        w.toString(),
        "-h",
        h.toString(),
        `svgs/${name}.svg`,
      ],
    });

    commands.push(command.output());
  }

  await Promise.all(commands);
};

const main = async () => {
  // const hidpi = true; //Deno.build.vendor === "apple";

  await Deno.remove(out, { recursive: true });
  await Deno.mkdir(out);

  const cmds: Promise<Deno.CommandOutput>[] = [];

  for (let index = 0; index < colors.length; index++) {
    const color = colors[index];

    let cmd = new Deno.Command("inkscape", {
      args: [
        `--export-filename=${out}/${color}_kodama.png`,
        "-w",
        dotSize.toString(),
        "-h",
        dotSize.toString(),
        `svgs/${color}_kodama.svg`,
      ],
    });

    cmds.push(cmd.output());

    cmd = new Deno.Command("inkscape", {
      args: [
        `--export-filename=${out}/${color}_dot.png`,
        "-w",
        dotSize.toString(),
        "-h",
        dotSize.toString(),
        `svgs/${color}_dot.svg`,
      ],
    });

    cmds.push(cmd.output());
  }

  const cmd = new Deno.Command("inkscape", {
    args: [
      `--export-filename=${out}/connector.png`,
      "-w",
      connectorSize.toString(),
      "-h",
      connectorSize.toString(),
      `svgs/connector.svg`,
    ],
  });

  cmds.push(cmd.output());

  await Promise.all(cmds);
};

await main();
await numbers();
await texts();
await buttonTransforms();
