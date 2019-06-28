#!/usr/bin/env node
"use strict"

let BinWrapper = require("bin-wrapper")
let path = require("path")
let version = require("./package.json").version

function createUrl(build) {
  return `https://github.com/jamiebuilds/codeowners-enforcer/releases/download/v${version}/codeowners-enforcer-v${version}-${build}.tar.gz`
}

let bin = new BinWrapper()
	.src(createUrl("x86_64-apple-darwin"), "darwin")
	.src(createUrl("x86_64-unknown-linux-musl"), "linux", "x64")
	.src(createUrl("x86_64-pc-windows-msvc"), "win32", "x64")
  .dest(path.resolve(__dirname, "vendor"))
  .use(process.platform === "win32" ? "codeowners-enforcer.exe" : "codeowners-enforcer")
  .version(version)

async function main() {
  await bin.run(["--version"])
}

main().catch(err => {
  console.error("Installation Failed:")
  console.error(err)
  process.exit(1)
})
