const bigDir = "bigimgs";
const out = "imgs";

const hidpi = true; //Deno.build.vendor === "apple";
const size = 56;
const connectorSize = 20;
const btnSize = 160;
const btnDims = `${btnSize}x${btnSize}`;

const dotSize = size * 2;
const dims = `${dotSize}x${dotSize}`;
const conSize = hidpi ? connectorSize * 2 : connectorSize;
const connectorDims = `${conSize}x${conSize}`;

const getDimensions = (name: string): string => {
  if (name === "connector.png") {
    return connectorDims;
  } else if (/plus|minus/.test(name)) {
    return btnDims;
  } else {
    return dims;
  }
};

const main = async () => {
  // const hidpi = true; //Deno.build.vendor === "apple";

  await Deno.remove(out, { recursive: true });
  await Deno.mkdir(out);

  const files = Deno.readDirSync("bigimgs");

  const cmds: Array<Promise<Deno.ProcessStatus>> = [];

  for (const img of files) {
    // convert original.png -resize 100x100 new.png
    console.log(img);

    const name = img.name;

    const dimensions = getDimensions(name);

    const cmd = `convert ${bigDir}/${name} -resize ${dimensions} ${out}/${name}`;
    const c = Deno.run({ cmd: cmd.split(" ") }).status();
    cmds.push(c);
  }

  await Promise.all(cmds);
};

await main();
