import { readFile } from 'node:fs/promises';
import { resolve } from 'node:path';
const readInput = async (path) => {
    const pat = resolve(path);
    const content = await readFile(pat, 'utf8');
    return content;
};
class Solution {
    input;
    constructor(input) {
        this.input = input;
    }
    part1() {
        return this.input.split('').reduce((acc, ch) => {
            switch (ch) {
                case "(":
                    acc += 1;
                    break;
                case ")":
                    acc -= 1;
                    break;
                default: throw new Error("Unknown char");
            }
            return acc;
        }, 0);
    }
    part2() {
    }
}
async function main() {
    let inp = await readInput('./input');
    let s = new Solution(inp);
    console.log(s.part1);
}
main();
