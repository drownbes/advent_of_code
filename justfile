prepare-aoc-day year day:
  mkdir -p {{year}}
  (cd {{year}} &&  cargo new --bin _{{day}} --name _{{year}}_{{day}})
  (cd {{year}}/_{{day}} && aoc -s ../../.adventofcode.session -y {{year}} -d {{day}} download)
  

aoc args:
  aoc -s .adventofcode.session {{args}}
