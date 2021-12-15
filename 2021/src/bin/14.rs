use std::collections::HashMap;

static INPUT: &str = "CKKOHNSBPCPCHVNKHFFK\n\nKO -> C\nSO -> S\nBF -> V\nVN -> B\nOV -> K\nVH -> O\nKV -> N\nKB -> F\nNB -> C\nHS -> K\nPF -> B\nHB -> N\nOC -> H\nFS -> F\nVV -> S\nKF -> C\nFN -> F\nKP -> S\nHO -> N\nNH -> K\nOO -> S\nFB -> C\nBP -> F\nCH -> N\nSN -> O\nKN -> B\nCV -> O\nCC -> B\nVB -> C\nPH -> V\nCO -> K\nKS -> K\nBK -> N\nFH -> S\nPV -> H\nCB -> P\nFO -> F\nBB -> K\nOB -> C\nHH -> F\nON -> O\nFK -> B\nNF -> F\nSV -> F\nCP -> H\nSS -> B\nOP -> H\nNS -> O\nHK -> N\nBC -> P\nNV -> V\nVS -> F\nPC -> V\nCS -> F\nNP -> V\nPS -> F\nVC -> F\nKK -> S\nPO -> P\nHF -> H\nKC -> P\nSF -> N\nBV -> N\nFF -> V\nFV -> V\nBO -> N\nOS -> C\nOF -> H\nCN -> S\nNO -> O\nNC -> B\nVK -> C\nHN -> B\nPK -> N\nSK -> S\nHV -> F\nBH -> B\nOK -> S\nVO -> B\nBS -> H\nPP -> N\nSC -> K\nBN -> P\nFC -> S\nSB -> B\nSH -> H\nNN -> V\nNK -> N\nVF -> H\nCF -> F\nPB -> C\nSP -> P\nKH -> C\nVP -> N\nCK -> H\nHP -> P\nFP -> B\nHC -> O\nPN -> F\nOH -> H";

fn solve(template: &str, rules: &HashMap<(&str, &str), &str>, iterations: usize) -> u64 {
    let len = template.len();
    let mut pair_count = HashMap::new();
    for i in 0..len - 1 {
        *pair_count
            .entry((&template[i..i + 1], &template[i + 1..i + 2]))
            .or_insert(0) += 1;
    }
    for _ in 0..iterations {
        for ((a, b), count) in pair_count.clone() {
            if let Some(c) = rules.get(&(a, b)) {
                *pair_count.entry((a, c)).or_insert(0) += count;
                *pair_count.entry((c, b)).or_insert(0) += count;
                *pair_count.get_mut(&(a, b)).unwrap() -= count;
            }
        }
    }
    let mut char_count = HashMap::new();
    for (&(a, _), count) in &pair_count {
        *char_count.entry(a).or_insert(0) += count;
    }
    *char_count.get_mut(&template[len - 1..len]).unwrap() += 1;
    char_count.values().max().unwrap() - char_count.values().min().unwrap()
}

fn main() {
    let mut lines = INPUT.lines();
    let template = lines.next().unwrap();
    let rules: HashMap<(&str, &str), &str> = lines
        .skip(1)
        .map(|line| {
            let (from, to) = line.split_once(" -> ").unwrap();
            ((&from[0..1], &from[1..2]), to)
        })
        .collect();
    println!("Part one: {}", solve(template, &rules, 10));
    println!("Part two: {}", solve(template, &rules, 40));
}
