use std::collections::{HashMap, VecDeque};

static INPUT: &str = "CKKOHNSBPCPCHVNKHFFK\n\nKO -> C\nSO -> S\nBF -> V\nVN -> B\nOV -> K\nVH -> O\nKV -> N\nKB -> F\nNB -> C\nHS -> K\nPF -> B\nHB -> N\nOC -> H\nFS -> F\nVV -> S\nKF -> C\nFN -> F\nKP -> S\nHO -> N\nNH -> K\nOO -> S\nFB -> C\nBP -> F\nCH -> N\nSN -> O\nKN -> B\nCV -> O\nCC -> B\nVB -> C\nPH -> V\nCO -> K\nKS -> K\nBK -> N\nFH -> S\nPV -> H\nCB -> P\nFO -> F\nBB -> K\nOB -> C\nHH -> F\nON -> O\nFK -> B\nNF -> F\nSV -> F\nCP -> H\nSS -> B\nOP -> H\nNS -> O\nHK -> N\nBC -> P\nNV -> V\nVS -> F\nPC -> V\nCS -> F\nNP -> V\nPS -> F\nVC -> F\nKK -> S\nPO -> P\nHF -> H\nKC -> P\nSF -> N\nBV -> N\nFF -> V\nFV -> V\nBO -> N\nOS -> C\nOF -> H\nCN -> S\nNO -> O\nNC -> B\nVK -> C\nHN -> B\nPK -> N\nSK -> S\nHV -> F\nBH -> B\nOK -> S\nVO -> B\nBS -> H\nPP -> N\nSC -> K\nBN -> P\nFC -> S\nSB -> B\nSH -> H\nNN -> V\nNK -> N\nVF -> H\nCF -> F\nPB -> C\nSP -> P\nKH -> C\nVP -> N\nCK -> H\nHP -> P\nFP -> B\nHC -> O\nPN -> F\nOH -> H";

/*
static INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
*/

fn solve(template: String, rules: &HashMap<&str, &str>, iterations: usize) -> u64 {
    let mut count = HashMap::new();
    let mut q1: VecDeque<String> = VecDeque::new();
    let mut q2: VecDeque<String> = VecDeque::new();
    for i in 0..template.len() {
        *count.entry(&template[i..i + 1]).or_insert(0) += 1;
        if i != template.len() - 1 {
            q1.push_back((&template[i..i + 2]).to_string());
        }
    }
    for _ in 0..iterations {
        let (q_full, q_empty) = if q1.is_empty() {
            (&mut q2, &mut q1)
        } else {
            (&mut q1, &mut q2)
        };
        while let Some(pair) = q_full.pop_front() {
            if let Some(letter) = rules.get(pair.as_str()) {
                *count.entry(letter).or_insert(0) += 1;
                q_empty.push_back(format!("{}{}", &pair[0..1], letter));
                q_empty.push_back(format!("{}{}", letter, &pair[1..2]));
            }
        }
    }
    count.values().max().unwrap() - count.values().min().unwrap()
}

fn main() {
    let mut lines = INPUT.lines();
    let template = lines.next().unwrap();
    let rules: HashMap<&str, &str> = lines
        .skip(1)
        .map(|line| line.split_once(" -> ").unwrap())
        .collect();
    println!("Part one: {}", solve(template.to_string(), &rules, 10));
    // println!("Part two: {}", solve(template.to_string(), &rules, 40));
}
