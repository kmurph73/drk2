import * as mod from "https://deno.land/std@0.185.0/fs/mod.ts";
const bigDir = "bigimgs";
const out = "imgs";

const main = async () => {
  const is_mac = Deno.build.vendor === "apple";

  const outDirExists = mod.existsSync(out);

  if (!outDirExists) {
    await Deno.mkdir(out);
  }

  const files = Deno.readDirSync("bigimgs");

  const cmds: Array<Promise<Deno.ProcessStatus>> = [];
  const dims = is_mac ? "120x120" : "60x60";

  for (const img of files) {
    // convert original.png -resize 100x100 new.png
    console.log(img);

    const name = img.name;

    const cmd = `convert ${bigDir}/${name} -resize ${dims} ${out}/${name}`;
    const c = Deno.run({ cmd: cmd.split(" ") }).status();
    cmds.push(c);
  }

  await Promise.all(cmds);
};

await main();
