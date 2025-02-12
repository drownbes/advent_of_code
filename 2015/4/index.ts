import { createHash } from 'node:crypto';

function calcMD5Hash(str:string) : string  {
}

class MD5Hash {
  hash: string;
  constructor(str: string) {
    this.hash = createHash('md5').update(str).digest("hex");
  }
  
  isStartsWithNZeros(n: number): boolean {
    return this.hash.startsWith("0".repeat(n));
  }
}



class Solution {
  input: string;
  constructor(input: string) {
    this.input = input;
  }

  findWithNZeros(zeros: number) : number {
    let n = 0;
    while(true) {
      const str = `${this.input}${n}`;
      const h = new MD5Hash(str);
      if (h.isStartsWithNZeros(zeros)) {
        return n;
      }
      n++;
    }
  }

  part1(): number {
    return this.findWithNZeros(5); 
  }

  part2() {
    return this.findWithNZeros(6); 
  }
}

async function main() {
  let s = new Solution("bgvyzdsv");

  console.log(s.part1());
  console.log(s.part2());
}

main()
