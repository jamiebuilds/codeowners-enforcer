#!/usr/bin/env node
"use strict"

let spawn = require("child_process").spawn
let bin = require("./bin")

let input = process.argv.slice(2)

spawn(bin.path(), input, { stdio: "inherit"})
  .on("exit", process.exit)
