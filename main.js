// import * as gol from './pkg/wasm_game_of_life.js';
// import gol from './pkg/wasm_game_of_life.js';
// import * from './pkg/wasm_game_of_life.js';

// https://stackoverflow.com/a/48952855/23059
// const { __wasm:gol } = require('./pkg/wasm_game_of_life.js');
// gol.greet();

const { greet } = require('./pkg/wasm_game_of_life.js');
greet();