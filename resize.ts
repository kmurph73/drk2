const bigDir = "bigimgs";
const out = "imgs";

const hidpi = true; //Deno.build.vendor === "apple";
const size = 40;
const connectorSize = size * 0.28;
const btnSize = 160;
const btnDims = `${btnSize}x${btnSize}`;

const dotSize = size * 2;
const dims = `${dotSize}x${dotSize}`;
let conSize = hidpi ? connectorSize * 2 : connectorSize;
conSize = Math.floor(conSize);

const connectorDims = `${conSize}x${conSize}`;

const getDimensions = (name: string): string | null => {
  if (name === "connector.png") {
    return connectorDims;
  } else if (/plus|minus/.test(name)) {
    return btnDims;
  } else if (/kodama|dot/.test(name)) {
    return dims;
  }

  return null;
};

const ignore = [".DS_Store"];

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

    if (ignore.includes(name)) {
      continue;
    }

    const cmd = (() => {
      const dimensions = getDimensions(name);

      if (dimensions) {
        return `convert ${bigDir}/${name} -resize ${dimensions} ${out}/${name}`;
      } else {
        return `cp ${bigDir}/${name} ${out}/${name}`;
      }
    })();

    const c = Deno.run({ cmd: cmd.split(" ") }).status();
    cmds.push(c);
  }

  await Promise.all(cmds);
};

await main();
