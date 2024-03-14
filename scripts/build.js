const { execSync } = require("child_process");

const options = {
  stdio: "inherit",
};
const target = process.env.target;
execSync(
  ` napi build --platform --release --js-package-name memoryshare ${target ? `--target ${target}` : ""}`,
  options,
);
