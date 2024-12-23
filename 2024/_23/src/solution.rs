use std::collections::{HashMap, HashSet};

pub fn solve_part1(strs: &[&str]) -> usize {
    let g = parse_input(strs);
    count_triplets(&g)
}

pub fn solve_part2(strs: &[&str]) -> String {
    let g = parse_input(strs);
    find_max_cliques(&g)
}

fn _dfs_cycle<'a>(
    u: &'a str,
    p: &'a str,
    graph: &HashMap<&'a str, HashSet<&'a str>>,
    color: &mut HashMap<&'a str, usize>,
    parents: &mut HashMap<&'a str, &'a str>,
    cycles: &mut Vec<Vec<&'a str>>,
) {
    if let Some(2) = color.get(&u) {
        return;
    }

    if let Some(1) = color.get(&u) {
        let mut v = vec![];
        let mut cur = p;
        v.push(cur);

        while cur != u {
            cur = parents.get(cur).unwrap();
            v.push(cur);
        }
        cycles.push(v);
        return;
    }

    parents.insert(u, p);
    color.insert(u, 1);

    let adj = graph.get(u).unwrap().clone();
    for v in adj {
        if parents.get(u).unwrap().eq(&v) {
            continue;
        }
        _dfs_cycle(v, u, graph, color, parents, cycles);
    }

    color.insert(u, 2);
}

fn _find_cycles<'a>(graph: &HashMap<&'a str, HashSet<&'a str>>) -> Vec<Vec<&'a str>> {
    let mut cycles = vec![];
    let mut i = graph.keys();
    let p = i.next().unwrap();
    let u = i.next().unwrap();

    _dfs_cycle(
        u,
        p,
        graph,
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut cycles,
    );
    cycles
}

fn parse_input<'a>(strs: &'a [&'a str]) -> HashMap<&'a str, HashSet<&'a str>> {
    strs.iter().fold(HashMap::new(), |mut acc, s| {
        let mut ss = s.split("-");
        let a = ss.next().unwrap();
        let b = ss.next().unwrap();

        acc.entry(a)
            .and_modify(|x| {
                x.insert(b);
            })
            .or_insert(HashSet::from([b]));
        acc.entry(b)
            .and_modify(|x| {
                x.insert(a);
            })
            .or_insert(HashSet::from([a]));
        acc
    })
}

fn count_triplets<'a>(graph: &'a HashMap<&'a str, HashSet<&'a str>>) -> usize {
    let mut rs: HashSet<Vec<&str>> = HashSet::new();
    for (frst, v) in graph.iter() {
        if frst.starts_with("t") {
            for snd in v.iter() {
                for thrd in graph.get(snd).unwrap().iter() {
                    if frst != thrd && graph.get(thrd).unwrap().contains(frst) {
                        let mut v = vec![*frst, *snd, *thrd];
                        v.sort();
                        rs.insert(v);
                    }
                }
            }
        }
    }
    rs.len()
}

fn bron_kerbosch<'a>(
    r: &mut HashSet<&'a str>,
    p: &mut HashSet<&'a str>,
    x: &mut HashSet<&'a str>,
    graph: &HashMap<&'a str, HashSet<&'a str>>,
) -> HashSet<Vec<&'a str>> {
    let mut cliques: HashSet<Vec<&str>> = HashSet::new();
    if p.is_empty() && x.is_empty() {
        let mut v: Vec<&str> = r.clone().into_iter().collect();
        v.sort();
        cliques.insert(v);
    }

    for v in p.clone() {
        let mut new_r = r.clone();
        new_r.insert(v);
        let mut new_p = p.clone();
        new_p.retain(|n| graph.get(v).unwrap().contains(*n));
        let mut new_x = x.clone();
        new_x.retain(|n| graph.get(v).unwrap().contains(*n));
        cliques.extend(bron_kerbosch(&mut new_r, &mut new_p, &mut new_x, graph));
        p.remove(v);
        x.insert(v);
    }
    cliques
}

fn find_max_cliques<'a>(graph: &'a HashMap<&'a str, HashSet<&'a str>>) -> String {
    let mut verts: HashSet<&str> = HashSet::from_iter(graph.keys().cloned());
    let cls = bron_kerbosch(&mut HashSet::new(), &mut verts, &mut HashSet::new(), graph);
    let max_clq = cls.iter().max_by_key(|x| x.len()).unwrap();
    max_clq.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
        kh-tc
        qp-kh
        de-cg
        ka-co
        yn-aq
        qp-ub
        cg-tb
        vc-aq
        tb-ka
        wh-tc
        yn-cg
        kh-ub
        ta-co
        de-co
        tc-td
        tb-wq
        wh-td
        ta-ka
        td-qp
        aq-cg
        wq-ub
        ub-vc
        de-ta
        wq-aq
        wq-vc
        wh-yn
        ka-de
        kh-ta
        co-tc
        wh-qp
        tb-vc
        td-yn";

    fn read_input(inp: &str) -> Vec<&str> {
        inp.strip_prefix("\n")
            .unwrap()
            .lines()
            .map(str::trim)
            .collect()
    }

    #[test]
    fn test_name() {
        let input = read_input(EXAMPLE);
        let res = solve_part1(&input);
        dbg!(res);
    }
}
