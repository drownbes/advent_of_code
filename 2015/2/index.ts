import { readFile } from 'node:fs/promises';
import { resolve } from 'node:path';


const readInput = async (path: string) : Promise<string> => {
  const pat = resolve(path);
  const content = await readFile(pat, 'utf8');
  return content;
};

class Box {
  l: number;
  w: number;
  h: number;
  constructor(str: string) {
    const parts = str.split('x');
    this.l = parseInt(parts[0]);
    this.w = parseInt(parts[1]);
    this.h = parseInt(parts[2]);
  }
  
  sortedSides() {
    const {l,w,h} = this;
    return [l,w,h].toSorted((a, b) => a - b);
  }
  
  square() {
    const {l,w,h} = this;
    const baseSq = 2*l*w + 2*w*h + 2*h*l
    const smallest = this.sortedSides();
    const appSq = smallest[0] * smallest[1];
    return baseSq + appSq;
  }

  ribbon() {
    const {l,w,h} = this;
    const vol = l * w * h;
    const smallest = this.sortedSides();
    const wrap = smallest[0] * 2 + smallest[1] * 2;
    return vol + wrap;
  }
}


class Solution {
  inputLines: string[];
  constructor(input: string) {
    this.inputLines = input.split(/\r?\n/).filter(x => x);
  }

  toBoxes(): Box[] {
    return this.inputLines.map(x => new Box(x));
  }

  part1(): number {
    const boxes = this.toBoxes();
    return boxes.map(b => b.square()).reduce((a, b) => a + b, 0);
  }

  part2() {
    const boxes = this.toBoxes();
    return boxes.map(b => b.ribbon()).reduce((a, b) => a + b, 0);
  }
}

async function main() {
  const inp = await readInput('./input');
  let s = new Solution(inp);

  console.log(s.part1());
  console.log(s.part2());
}

main()
