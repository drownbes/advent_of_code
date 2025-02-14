import { readFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { inspect } from 'node:util';


const readInput = async (path: string) : Promise<string> => {
  const pat = resolve(path);
  const content = await readFile(pat, 'utf8');
  return content;
};

interface Cord {
  x: number;
  y: number;
}

class RectRange {
  start: Cord;
  end: Cord;
  constructor(start: Cord, end: Cord) {
    this.start = start;
    this.end = end;
  }

  *iter() {
    let start = this.start;
    let end = this.end;
    for(let y=start.y; y <= end.y;y++) {
      for(let x=start.x; x <= end.x;x++) {
        yield ({x, y});
      }
    }
    return null;
  }
}


class InstructionParser {
  static parse(str: string): ActionInterface {
    let parts = str.split(' ').entries();
    let first = parts.next().value![1];
    switch (first) {
      case "toggle": return new Toggle(InstructionParser.parseRange(parts));
      case "turn": 
        let act = parts.next().value![1];
        switch (act) {
          case "on": return new TurnOn(InstructionParser.parseRange(parts));
          case "off": return new TurnOff(InstructionParser.parseRange(parts));
          default: throw Error("Failed to Parse");
        }
      default: throw Error("Failed to Parse");
    }
  }

  static parseRange(iter: ArrayIterator<[number, string]>): RectRange {
    let s = iter.next().value![1];
    let [startX, startY] = s.split(",").map(x => parseInt(x.trim()));
    iter.next();
    let [endX, endY] = iter.next().value![1].split(",").map(x => parseInt(x.trim()));
    return (new RectRange(
      {
        x: startX,
        y: startY
      },
      {
        x: endX,
        y: endY
      }
    ));
  }
}

interface ActionContructor {
  new (range: RectRange): ActionInterface;
};

interface ActionInterface {
  range: RectRange;
  act(l: Light): void;
}

const TurnOn: ActionContructor = class TurnOn implements ActionInterface {
  constructor(public range: RectRange) {
    this.range = range;
  }
  act(l: Light) {
    l.turnOn();
  }
}

const TurnOff: ActionContructor = class TurnOff implements ActionInterface {
  constructor(public range: RectRange) {}
  act(l: Light) {
    l.turnOff();
  }
}

const Toggle: ActionContructor = class Toggle implements ActionInterface {
  constructor(public range: RectRange) {}
  act(l: Light) {
    l.toggle();
  }
}

class Grid {
  grid: Array<Array<Light>>;
  constructor(w: number, h: number, t: new() => Light) {
    this.grid = Array(h).fill(0).map(_ => Array(w).fill(0).map(_ => new t()));
  }

  applyAction(action: ActionInterface) {
    for (let {x, y} of action.range.iter()) {
      action.act(this.grid[y][x]);
    }
  }

  lightsValue() {
    let count = 0;
    for (let row of this.grid) {
      for (let col of row) {
        count += col.getValue();
      }
    }
    return count;
  }
}


interface Light {
  toggle():void;
  turnOn():void;
  turnOff():void;
  getValue(): number;
}

class BinaryLight implements Light {
  value: boolean;
  constructor() {
    this.value = false;
  }
  toggle(): void {
    this.value = !this.value;
  }
  turnOn(): void {
    this.value = true;
  }
  turnOff(): void {
    this.value = false;
  }
  getValue(): number {
    return this.value ? 1 : 0;
  }
}

class IntLight implements Light {
  value: number;
  constructor() {
    this.value = 0;
  }
  toggle(): void {
    this.value+=2;
  }
  turnOn(): void {
    this.value++;
  }
  turnOff(): void {
    if (this.value > 0) {
      this.value--;
    }
  }
  getValue(): number {
    return this.value;
  }
}


class Solution {
  inputLines: string[];
  instructions: Array<ActionInterface>;
  constructor(input: string) {
    this.inputLines = input.split(/\r?\n/).filter(x => x);
    this.instructions = this.inputLines.map(InstructionParser.parse);
  }

  solve(t: new() => Light): number {
    let grid = new Grid(1000,1000, t);
    for (let action of this.instructions) {
      grid.applyAction(action);
    }
    return grid.lightsValue();
  }

  part1() {
    return this.solve(BinaryLight);
  }

  part2() {
    return this.solve(IntLight);
  }
}

async function main() {
  const inp = await readInput('./input');
  let s = new Solution(inp);
  console.log(s.part1());
  console.log(s.part2());
}

main()
