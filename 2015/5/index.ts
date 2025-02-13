import { readFile } from 'node:fs/promises';
import { resolve } from 'node:path';


const readInput = async (path: string) : Promise<string> => {
  const pat = resolve(path);
  const content = await readFile(pat, 'utf8');
  return content;
};

class NiceDetector {
  vowels = new Set("aeiou".split(''));
  forbidden = ["ab", "cd", "pq", "xy"];
  input: string[];

  constructor(input: string) {
    this.input = input.split('');
  }

  isVowel(ch: string) {
    return this.vowels.has(ch);
  }

  isForbidden(s: string) {
    return this.forbidden.includes(s);
  }

  check() {
    let hasDouble = false;
    let vowelsCount = 0;
    let hasForbidden = false;
    if (this.isVowel(this.input[0])) {
      vowelsCount++;
    }

    for(let i = 1; i < this.input.length;i++)  {
      const ch0 = this.input[i-1];
      const ch1 = this.input[i];
      if (ch0 == ch1) {
        hasDouble = true;
      }
      if (this.isVowel(ch1)) {
        vowelsCount++;
      }

      if (this.isForbidden(ch0 + ch1)) {
        hasForbidden = true;
        break;
      }
    }

    return hasDouble && (vowelsCount >= 3) && !hasForbidden;
  }
}

class NiceDetectorFixed {
  input: string[];

  constructor(input: string) {
    this.input = input.split('');
  }

  check() {
    let hasRepeatsWithOneLetter = false;
    let metTwoLetters : Map<string, Array<[number, number]>> = new Map();
    let hasTwicePair = false;

    const hasNonOverlapping = (ls: string, indexes: [number, number]) => {
      let met = metTwoLetters.get(ls); 
      let res = false;
      if (met) {
        res =  met.find((xs) => indexes.every(x => !xs.includes(x))) !== undefined;
      }
      metTwoLetters.set(ls, (met || []).concat([indexes]));
      return res;
    };

    metTwoLetters.set(this.input[0] + this.input[1], [[0, 1]]);

    for(let i = 0; i < this.input.length - 2;i++)  {
      const ch0 = this.input[i];
      const ch1 = this.input[i+1];
      const ch2 = this.input[i+2];
      if(ch0 == ch2) {
        hasRepeatsWithOneLetter = true;
        //console.log(ch0 + ch1 + ch2);
      }

      let twoLetters = ch1 + ch2;
      if (hasNonOverlapping(twoLetters, [i+1, i+2])) {
        hasTwicePair = true;
      }

      if (hasTwicePair && hasRepeatsWithOneLetter) {
        return true; 
      }
    }
    return false;
  }
}



class Solution {
  inputLines: string[];
  constructor(input: string) {
    this.inputLines = input.split(/\r?\n/).filter(x => x);
  }

  part1() {
    return this.inputLines.reduce((acc, line) => {
      if ((new NiceDetector(line)).check()) {
        acc++;
      }
      return acc;
    }, 0);
  }

  part2() {
    return this.inputLines.reduce((acc, line) => {
      if ((new NiceDetectorFixed(line)).check()) {
        acc++;
      }
      return acc;
    }, 0);

  }
}

async function main() {
  const inp = await readInput('./input');
  let s = new Solution(inp);

  console.log(s.part1());
  console.log(s.part2());

  //console.log((new NiceDetectorFixed("qjhvhtzxzqqjkmpb").check()));
  //console.log((new NiceDetectorFixed("xxyxx").check()));
  //console.log((new NiceDetectorFixed("uurcxstgmygtbstg").check()));
  //console.log((new NiceDetectorFixed("ieodomkazucvgmuy").check()));
}

main()
