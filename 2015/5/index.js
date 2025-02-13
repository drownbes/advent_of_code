import { readFile } from 'node:fs/promises';
import { resolve } from 'node:path';
const readInput = async (path) => {
    const pat = resolve(path);
    const content = await readFile(pat, 'utf8');
    return content;
};
class NiceDetector {
    vowels = "aeiou";
    forbidden = ["ab", "cd", "pq", "xy"];
    input;
    constructor(input) {
        this.input = input.split('');
    }
    check() {
        let isVowel = this.vowels.contains(this.input[0]);
        for (const ch of this.input) {
        }
    }
}
class Solution {
    input;
    constructor(input) {
        this.input = input.split('');
        console.log(this.input);
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
