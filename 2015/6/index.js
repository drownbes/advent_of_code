import { readFile } from 'node:fs/promises';
import { resolve } from 'node:path';
const readInput = async (path) => {
    const pat = resolve(path);
    const content = await readFile(pat, 'utf8');
    return content;
};
class RectRange {
    start;
    end;
    constructor(start, end) {
        this.start = start;
        this.end = end;
    }
    *iter() {
        let start = this.start;
        let end = this.end;
        for (let y = start.y; y <= end.y; y++) {
            for (let x = start.x; x <= end.x; x++) {
                yield ({ x, y });
            }
        }
        return null;
    }
}
class InstructionParser {
    static parse(str) {
        let parts = str.split(' ').entries();
        let first = parts.next().value[1];
        switch (first) {
            case "toggle": return new Toggle(InstructionParser.parseRange(parts));
            case "turn":
                let act = parts.next().value[1];
                switch (act) {
                    case "on": return new TurnOn(InstructionParser.parseRange(parts));
                    case "off": return new TurnOff(InstructionParser.parseRange(parts));
                    default: throw Error("Failed to Parse");
                }
            default: throw Error("Failed to Parse");
        }
    }
    static parseRange(iter) {
        let s = iter.next().value[1];
        let [startX, startY] = s.split(",").map(x => parseInt(x.trim()));
        iter.next();
        let [endX, endY] = iter.next().value[1].split(",").map(x => parseInt(x.trim()));
        return (new RectRange({
            x: startX,
            y: startY
        }, {
            x: endX,
            y: endY
        }));
    }
}
;
const TurnOn = class TurnOn {
    range;
    constructor(range) {
        this.range = range;
        this.range = range;
    }
    act(l) {
        l.turnOn();
    }
};
const TurnOff = class TurnOff {
    range;
    constructor(range) {
        this.range = range;
    }
    act(l) {
        l.turnOff();
    }
};
const Toggle = class Toggle {
    range;
    constructor(range) {
        this.range = range;
    }
    act(l) {
        l.toggle();
    }
};
class Grid {
    grid;
    constructor(w, h, t) {
        this.grid = Array(h).fill(0).map(_ => Array(w).fill(new t()));
    }
    applyAction(action) {
        for (let { x, y } of action.range.iter()) {
            action.act(this.grid[y][x]);
        }
    }
    lightsValue() {
        let count = 0;
        for (let row of this.grid) {
            for (let col of row) {
                if (col)
                    count += col.getValue();
            }
        }
        return count;
    }
}
class BinaryLight {
    value;
    constructor() {
        this.value = false;
    }
    toggle() {
        this.value = !this.value;
    }
    turnOn() {
        this.value = true;
    }
    turnOff() {
        this.value = false;
    }
    getValue() {
        return this.value ? 1 : 0;
    }
}
class IntLight {
    value;
    constructor() {
        this.value = 0;
    }
    toggle() {
        this.value += 2;
    }
    turnOn() {
        this.value++;
    }
    turnOff() {
        this.value--;
    }
    getValue() {
        return this.value;
    }
}
class Solution {
    inputLines;
    instructions;
    constructor(input) {
        this.inputLines = input.split(/\r?\n/).filter(x => x);
        this.instructions = this.inputLines.map(InstructionParser.parse);
    }
    solve(t) {
        let grid = new Grid(1000, 1000, t);
        for (let action of this.instructions) {
            grid.applyAction(action);
            console.log(grid);
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
main();
