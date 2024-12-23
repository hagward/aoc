import std/[algorithm, sequtils, sets, strutils, sugar, tables]

let input = readAll(stdin).splitLines

var neighbors: Table[string, HashSet[string]]
for line in input:
    let (a, b) = (line[0..1], line[3..4])
    if not neighbors.contains(a): neighbors[a] = initHashSet[string]()
    if not neighbors.contains(b): neighbors[b] = initHashSet[string]()
    neighbors[a].incl(b)
    neighbors[b].incl(a)

var groups: HashSet[(string, string, string)]
for a in neighbors.keys:
    for b in neighbors[a]:
        for c in neighbors[b]:
            if c in neighbors[a]:
                let group = sorted(@[a, b, c])
                groups.incl((group[0], group[1], group[2]))

echo groups.toSeq.filter(group => @[group[0], group[1], group[2]].any(node => node.startsWith("t"))).len

var cliques: seq[HashSet[string]]

proc bronKerbosch(r, p, x: HashSet[string]) =
    if p.len == 0 and x.len == 0:
        cliques.add(r)
    var
        rr = r
        pp = p
        xx = x
    for v in p:
        bronKerbosch(rr + [v].toHashSet, pp * neighbors[v], xx * neighbors[v])
        pp.excl(v)
        xx.incl(v)

bronKerbosch(initHashSet[string](), neighbors.keys.toSeq.toHashSet, initHashSet[string]())

echo sorted(cliques[cliques.mapIt(it.len).maxIndex].toSeq).join(",")
