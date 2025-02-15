import { readFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { inspect } from 'node:util';


const readInput = async (path: string) : Promise<string> => {
  const pat = resolve(path);
  const content = await readFile(pat, 'utf8');
  return content;
};


interface Op {
  output: string;

  tryToCalcOutput(doneOutputs: Map<string, number>): number | null;
}

class And implements Op {
  first: number | string;
  second: number | string;
  output: string;
  constructor(first: number | string, second: number | string, output: string) {
    this.first = first;
    this.second = second;
    this.output = output;
  }
}

class Or implements Op {
  first: number | string;
  second: number | string;
  output: string;
  constructor(first: number | string, second: number | string, output: string) {
    this.first = first;
    this.second = second;
    this.output = output;
  }

}

class LShift implements Op {
  first: number | string;
  second: number | string;
  output: string;
  constructor(first: number | string, second: number | string, output: string) {
    this.first = first;
    this.second = second;
    this.output = output;
  }
}

class RShift implements Op {
  first: number | string;
  second: number | string;
  output: string;
  constructor(first: number | string, second: number | string, output: string) {
    this.first = first;
    this.second = second;
    this.output = output;
  }
}


class Not implements Op {
  input: string;
  output: string;
  constructor(input: string, output: string) {
    this.input = input;
    this.output = output;
  }

}

class Value implements Op {
  input: number;
  output: string;
  constructor(input: number, output: string) {
    this.input = input;
    this.output = output;
  }

}

const twoInputs = /([a-z]+|[0-9]+)\s(LSHIFT|RSHIFT|AND|OR)\s([a-z]+|[0-9]+)\s->\s([a-z]+)/;
const not = /NOT\s([a-z]+)\s->\s([a-z]+)/;
const value = /([0-9])+\s->\s([a-z]+)/;
const id = /([a-z])+\s->\s([a-z]+)/;


class GateParser {
  static parse(str: string) {
    console.log(str);
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
    return new Value(parseInt(left), output);
  }
}

class Solution {
  inputLines: string[];
  constructor(input: string) {
    this.inputLines = input.split(/\r?\n/).filter(x => x);
  }

  part1() {
    let gates = this.inputLines.map(GateParser.parse);
    console.log(gates);
  }

  part2() {
  }
}

async function main() {
  const inp = await readInput('./input');
  let s = new Solution(inp);
  console.log(s.part1());
  console.log(s.part2());
}

main()
