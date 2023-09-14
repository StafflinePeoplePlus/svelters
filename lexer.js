const {Lexer} = require('html-lexer');
const fs = require("fs");

const delegate = {
  write: (token) => console.log (token),
  end: () => null
};

const lexer = new Lexer(delegate);
lexer.write(fs.readFileSync("./source.svelte"));
lexer.end();