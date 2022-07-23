import { memory } from "../pkg/index_bg";
import { Image } from "../pkg/index" ;

console.log(memory);

const image = Image.new();
const pixelsPointr = image.pixels_ptr();
const pixels = new Uint8Array(memory.buffer, pixelsPointr, 6);

console.log(pixels);

