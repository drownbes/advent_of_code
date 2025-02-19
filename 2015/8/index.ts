import { readFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { inspect } from 'node:util';
import assert, { throws } from 'node:assert';
import { unescape } from 'node:querystring';
import { isAscii, isUtf8 } from 'node:buffer';


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


function removeSurroundQuotes(value: Buffer) {
}

class UnscapedString {
  orig: Buffer;
  unescaped: string;
  constructor(public value: Buffer) {
    this.orig = value;
    this.unescaped = UnscapedString.unescape(value);
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

  diff() {
    return this.orig.length - this.unescaped?.length;
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
  }
}

async function main() {
  const inp = await readInput('./input');

  //console.log(Buffer.from('"aaa\\"aaa"'));
  //let s0 = new UnscapedString(Buffer.from('"aaa\\\"aaa"'));
  //console.log(s0.orig.length, s0.unescaped?.length, s0.unescaped);

  //let s1 = new UnscapedString(Buffer.from("\"abc\""));
  //console.log(s1.orig.length, s1.unescaped?.length, s1.unescaped);

  //console.log(Buffer.from('"\\x27"'));
  //let s2 = new UnscapedString(Buffer.from('"\\x27"'));
  //console.log(s2.orig.length, s2.unescaped?.length, s2.unescaped);

  //console.log(Buffer.from('""'));
  //let s3 = new UnscapedString(Buffer.from('""'));
  //console.log(s3.orig.length, s3.unescaped?.length, s3.unescaped);

  let s = new Solution(inp);
  console.log(s.part1());
  //console.log(s.part2());
}

main()
