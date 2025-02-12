import { readFile } from 'node:fs/promises';
import { resolve } from 'node:path';


const readInput = async (path: string) : Promise<string> => {
  const pat = resolve(path);
  const content = await readFile(pat, 'utf8');
  return content;
};

class Courier {
  x: number;
  y: number;
  constructor() {
    this.x = 0;
    this.y = 0;
  }

  hash() {
    return `${this.x}_${this.y}`;
  }

  step(move:string) {
    switch (move) {
        case '>': this.x+=1; break;
        case '<': this.x-=1; break;
        case '^': this.y-=1; break;
        case 'v': this.y+=1; break;
      }
  }
}

class Solution {
  input: string[];
  constructor(input: string) {
    this.input = input.split('');
    console.log(this.input);
  }

  part1(): number {
    const uniqHouses = new Set();
    let c = new Courier();
    uniqHouses.add("0_0");
    for (const move of this.input) {
      c.step(move); 
      uniqHouses.add(c.hash());
    }
    return uniqHouses.size;
  }

  part2() {
    const uniqHouses = new Set();
    uniqHouses.add("0_0");
    const santa = new Courier();
    const robot = new Courier();
    let active = santa;
    for (const move of this.input) {
      active.step(move);
      uniqHouses.add(active.hash());
      if (active == santa) {
        active = robot;
      } else {
        active = santa;
      }
    }
    return uniqHouses.size;
  }
}

async function main() {
  const inp = await readInput('./input');
  let s = new Solution(inp);

  console.log(s.part1());
  console.log(s.part2());
}

main()
