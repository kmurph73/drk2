import * as mod from "https://deno.land/std@0.185.0/fs/mod.ts";
const bigDir = "bigimgs";
const out = "imgs";

const size = 56;

const main = async () => {
  const is_mac = Deno.build.vendor === "apple";

  const outDirExists = mod.existsSync(out);

  if (!outDirExists) {
    await Deno.mkdir(out);
  }

  const files = Deno.readDirSync("bigimgs");

  const cmds: Array<Promise<Deno.ProcessStatus>> = [];
  const dotSize = is_mac ? size * 2 : size;
  const dims = `${dotSize}x${dotSize}`;
  const connectorSize = Math.floor(dotSize / 4);
  const connectorDims = `${connectorSize}x${connectorSize}`;

  for (const img of files) {
    // convert original.png -resize 100x100 new.png
    console.log(img);

    const name = img.name;

    const dimensions = name === "connector.png" ? connectorDims : dims;

    const cmd = `convert ${bigDir}/${name} -resize ${dimensions} ${out}/${name}`;
    const c = Deno.run({ cmd: cmd.split(" ") }).status();
    cmds.push(c);
  }

  await Promise.all(cmds);
};

await main();
