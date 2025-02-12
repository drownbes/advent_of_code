import { readFile } from 'node:fs/promises';
import { resolve } from 'node:path';


const readInput = async (path: string) : Promise<string> => {
  const pat = resolve(path);
  const content = await readFile(pat, 'utf8');
  return content;
};

class Solution {
  input: string[];
  constructor(input: string) {
    this.input = input.split('');
  }

  symOp(ch:string): (n: number) => number {
    switch (ch) {
        case "(": return (n:number) => n+1;
        case ")": return (n: number) => n-1;
        default: throw new Error("Unknown char");
    }
  }

  part1(): number {
    return this.input.reduce((acc, ch) => {
      return this.symOp(ch)(acc);
    }, 0);
  }

  part2(): number {
    let st = 0;
    for (const [index, ch] of this.input.entries()) {
      st = this.symOp(ch)(st);
      if (st < 0) {
        return index + 1;
      }
    }
    throw new Error("Not found");
  }
}

async function main() {
  const inp = await readInput('./input');
  let s = new Solution(inp);

  console.log(s.part1());
  console.log(s.part2());
}

main()
