import { readFile } from 'node:fs/promises';
import { resolve } from 'node:path';
const readInput = async (path) => {
    const pat = resolve(path);
    const content = await readFile(pat, 'utf8');
    return content;
};
class And {
    first;
    second;
    output;
    constructor(first, second, output) {
        this.first = first;
        this.second = second;
        this.output = output;
    }
}
class Or {
    first;
    second;
    output;
    constructor(first, second, output) {
        this.first = first;
        this.second = second;
        this.output = output;
    }
}
class LShift {
    first;
    second;
    output;
    constructor(first, second, output) {
        this.first = first;
        this.second = second;
        this.output = output;
    }
}
class RShift {
    first;
    second;
    output;
    constructor(first, second, output) {
        this.first = first;
        this.second = second;
        this.output = output;
    }
}
class Not {
    input;
    output;
    constructor(input, output) {
        this.input = input;
        this.output = output;
    }
}
class Value {
    input;
    output;
    constructor(input, output) {
        this.input = input;
        this.output = output;
    }
}
const twoInputs = /([a-z]{1}|[0-9]+)\s(LSHIFT|RSHIFT|AND|OR)\s([a-z]{1}|[0-9]+)\s->\s([a-z]{1})/;
const not = /NOT\s([a-z]{1})\s->\s([a-z]{1})/;
const value = /([0-9])+\s->\s[a-z]{1}/;
class GateParser {
    parse(str) {
        if (twoInputs.test(str)) {
            return GateParser.parseTwoInputs(str);
        }
        else if (not.test(str)) {
        }
        else if (value.test(str)) {
        }
        else {
            throw Error("Invalid gate");
        }
    }
    parseTwoInputs(str) {
        let [_, first, op, second, output] = twoInputs.exec(str);
        switch (op) {
            case "AND": return new And(first, second, output);
            case "OR": return new Or(first, second, output);
            case "LSHIFT": return new LShift(first, second, output);
            case "RSHIFT": return new RShift(first, second, output);
            default:
                throw Error(`Unknown 2 inputs op ${op}`);
        }
    }
    parseNot(str) {
        let [_, left, output] = not.exec(str);
        return new Not(left, output);
    }
    parseValue(str) {
        let [_, left, output] = value.exec(str);
        return new Value(parseInt(left), output);
    }
}
class Solution {
    inputLines;
    constructor(input) {
        this.inputLines = input.split(/\r?\n/).filter(x => x);
    }
    part1() {
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
main();
