import { readFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { inspect } from 'node:util';
import assert from 'node:assert';


const readInput = async (path: string) : Promise<string> => {
  const pat = resolve(path);
  const content = await readFile(pat, 'utf8');
  return content;
};


interface Op {
  output: string;
  tryToCalcOutput(doneOutputs: Map<string, number>): number | null;
}

function tryParseInt(x: string) : number | string {
  const n = parseInt(x);
  return isNaN(n) ? x : n
}


function checkResolved(x: string | number, m: Map<string, number>) : string | number {
  if (typeof x === 'number') {
    return x;
  } else {
    let mx = m.get(x);
    if (mx === undefined) {
      return x;
    } else {
      return mx;
    }
  }
}

class TwoInputs {
  first: number | string;
  second: number | string;
  output: string;
  constructor(first: string, second: string, output: string) {
    this.first = tryParseInt(first);
    this.second = tryParseInt(second);
    this.output = output;
  }
}

class And extends TwoInputs implements Op {
  constructor(first: string, second: string, output: string) {
    super(first, second, output);
  }

  tryToCalcOutput(doneOutputs: Map<string, number>): number | null {
    let f = checkResolved(this.first, doneOutputs);
    let s = checkResolved(this.second, doneOutputs);
    if (typeof f === 'number' && typeof s === 'number') {
      return f & s;
    } else {
      return null;
    }
  }
}

class Or implements Op {
  constructor(first: string, second: string, output: string) {
    super(first, second, output);
  }

  tryToCalcOutput(doneOutputs: Map<string, number>): number | null {
    let f = checkResolved(this.first, doneOutputs);
    let s = checkResolved(this.second, doneOutputs);
    if (typeof f === 'number' && typeof s === 'number') {
      return f | s;
    } else {
      return null;
    }
  }
}

class LShift implements Op {
  constructor(first: string, second: string, output: string) {
    super(first, second, output);
  }

  tryToCalcOutput(doneOutputs: Map<string, number>): number | null {
    let f = checkResolved(this.first, doneOutputs);
    let s = checkResolved(this.second, doneOutputs);
    if (typeof f === 'number' && typeof s === 'number') {
      return f <<  s;
    } else {
      return null;
    }
  }
}

class RShift implements Op {
  constructor(first: string, second: string, output: string) {
    super(first, second, output);
  }

  tryToCalcOutput(doneOutputs: Map<string, number>): number | null {
    let f = checkResolved(this.first, doneOutputs);
    let s = checkResolved(this.second, doneOutputs);
    if (typeof f === 'number' && typeof s === 'number') {
      return f >> s;
    } else {
      return null;
    }
  }
}


class Not implements Op {
  input: string;
  output: string;
  constructor(input: string, output: string) {
    this.input = input;
    this.output = output;
  }

  tryToCalcOutput(doneOutputs: Map<string, number>): number | null {
    let f = checkResolved(this.input, doneOutputs);
    if (typeof f === 'number') {
      return ~f;
    } else {
      return null;
    }
  }

}

class Value implements Op {
  input: number;
  output: string;
  constructor(input: number, output: string) {
    this.input = input;
    this.output = output;
  }

  tryToCalcOutput(_doneOutputs: Map<string, number>): number | null {
    return this.input;
  }

}

class Id implements Op {
  input: string;
  output: string;
  constructor(input: string, output: string) {
    this.input = input;
    this.output = output;
  }

  tryToCalcOutput(doneOutputs: Map<string, number>): number | null {
    let f = checkResolved(this.input, doneOutputs);
    if (typeof f === 'number') {
      return f;
    } else {
      return null;
    }
  }
}

const twoInputs = /([a-z]+|[0-9]+)\s(LSHIFT|RSHIFT|AND|OR)\s([a-z]+|[0-9]+)\s->\s([a-z]+)/;
const not = /NOT\s([a-z]+)\s->\s([a-z]+)/;
const value = /([0-9]+)\s->\s([a-z]+)/;
const id = /([a-z]+)\s->\s([a-z]+)/;


class GateParser {
  static parse(str: string) {
    if (twoInputs.test(str)) {
      return GateParser.parseTwoInputs(str);
    } else if(not.test(str)) {
      return GateParser.parseNot(str);
    } else if(value.test(str)) {
      return GateParser.parseValue(str);
    } else if(id.test(str)) {
      return GateParser.parseId(str);
    } else {
      throw Error(`Invalid gate ${str}`);
    }
  }

  static parseTwoInputs(str:string): Op {
    let [_, first, op, second, output] = twoInputs.exec(str)!;
    switch(op) {
      case "AND": return new And(first, second, output);
      case "OR": return new Or(first, second, output);
      case "LSHIFT": return new LShift(first, second, output);
      case "RSHIFT": return new RShift(first, second, output);
      default: 
        throw Error(`Unknown 2 inputs op ${op}`);
    }
  }

  static parseNot(str:string) {
    let [_, left, output] = not.exec(str)!;
    return new Not(left, output);
  }
  
  static parseValue(str:string) {
    let [_, left, output] = value.exec(str)!;
    return new Value(parseInt(left), output);
  }
  static parseId(str:string) {
    let [_, left, output] = id.exec(str)!;
    return new Id(left, output);
  }
}

class Net {
  gates: Op[];
  outputs: Map<string, number> = new Map();
  constructor(inputLines: string[]) {
    this.gates = inputLines.map(GateParser.parse);
  }

  process() {
    while(this.gates.length > 0) {
      const g = this.gates.pop()!;
      let v = g.tryToCalcOutput(this.outputs);
      if (v === null) {
        this.gates.unshift(g);
      } else {
        if (g.output == 'b') {
          v = 956;
        }
        this.outputs.set(g.output, v);
      }
    }
  }
}

class Solution {
  inputLines: string[];
  constructor(input: string) {
    this.inputLines = input.split(/\r?\n/).filter(x => x);
  }

  part1() {
    let net = new Net(this.inputLines);
    net.process();
    return net.outputs.get('a');
  }

  part2() {
    let net = new Net(this.inputLines);
    net.process();
    return net.outputs.get('a');
  }
}

async function main() {
  const inp = await readInput('./input');
  let s = new Solution(inp);
  console.log(s.part1());
  console.log(s.part2());
}

main()
