#!/usr/bin/env node
"use strict"

let bin = require("./bin")

bin.run(["--version"]).catch(err => {
  console.error(err)
  process.exit(1)
})
