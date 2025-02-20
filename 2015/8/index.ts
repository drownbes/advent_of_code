import { readFile } from 'node:fs/promises';
import { resolve } from 'node:path';

const readInput = async (path: string) : Promise<Buffer> => {
  const pat = resolve(path);
  const content = await readFile(pat);
  return content;
};

function splitNewLines(buffer: Buffer) : Buffer[] {
  let start = 0;
  let bufs = [];
  for (let i=0; i < buffer.length; i++) {
    if (buffer[i] === "\n".charCodeAt(0)) {
      bufs.push(buffer.subarray(start, i));
      start = i+1;
    }
  }
  return bufs;
}


class UnscapedString {
  orig: Buffer;
  unescaped: string;
  encoded: string;
  constructor(public value: Buffer) {
    this.orig = value;
    this.unescaped = UnscapedString.unescape(value);
    this.encoded = UnscapedString.encode(value)
  }

  static unescape(val: Buffer): string {
    let value = val.subarray(1, val.length - 1);
    let i = 0;
    let unescaped = "";
    while(i < value.length) {
      if(value[i] === "\\".charCodeAt(0)) {
        if(i+1 < value.length && value[i+1] === "\\".charCodeAt(0)) {
          unescaped+="\\";
          i+=2;
        } else if(i+1 < value.length && value[i+1] === "\"".charCodeAt(0)) {
          unescaped+="\"";
          i+=2;
        } else if(i+3 < value.length && value[i+1] === "x".charCodeAt(0)) {
          const hex = String.fromCharCode(value[i+2]) + String.fromCharCode(value[i+3]);
          let ch = String.fromCharCode(parseInt(hex, 16));
          unescaped+=ch;
          i+=4;
        } else {
          throw Error("Bad escape sequence");
        }
      } else {
        unescaped+=String.fromCharCode(value[i]);
        i++;
      }
    }
    return unescaped;
  }

  static encode(val: Buffer): string {
    let value = val.subarray(1, val.length - 1);
    let i = 0;
    let escaped = "";
    console.log(value);
    while(i < value.length) {
      if(value[i] === "\\".charCodeAt(0)) {
        if(i+1 < value.length && value[i+1] === "\\".charCodeAt(0)) {
          escaped +="\\\\\\\\";
          i+=2;
        } else if(i+1 < value.length && value[i+1] === "\"".charCodeAt(0)) {
          escaped+="\\\\\\\"";
          i+=2;
        } else if(i+3 < value.length && value[i+1] === "x".charCodeAt(0)) {
          const hex = String.fromCharCode(value[i+2]) + String.fromCharCode(value[i+3]);
          escaped +=  "\\\\x" + hex;
          i+=4;
        } else {
          escaped+="\\\\";
          i++;
        }
      } else {
        escaped +=String.fromCharCode(value[i]);
        i++;
      }
    }
    return "\"\\\"" + escaped + "\\\"\"";
  }

  diff() {
    return this.orig.length - this.unescaped?.length;
  }

  diff_encoded() {
    return this.encoded.length - this.orig.length;
  }
  
}


class Solution {
  bufs: Buffer[];

  constructor(input: Buffer) {
    this.bufs = splitNewLines(input);
  }

  part1() {
    return this.bufs.reduce((acc, b) => {
      acc += (new UnscapedString(b)).diff();
      return acc;
    }, 0);
  }

  part2() {
    return this.bufs.reduce((acc, b) => {
      acc += (new UnscapedString(b)).diff_encoded();
      return acc;
    }, 0);
  }
}

async function main() {
  const inp = await readInput('./input');
  let s = new Solution(inp);
  console.log(s.part1());
  console.log(s.part2());
}

main()
