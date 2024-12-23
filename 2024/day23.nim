import std/[algorithm, sequtils, sets, strutils, sugar, tables]

let input = readAll(stdin).splitLines

var graph: Table[string, HashSet[string]]
for line in input:
    let (a, b) = (line[0..1], line[3..4])
    if not graph.contains(a): graph[a] = initHashSet[string]()
    if not graph.contains(b): graph[b] = initHashSet[string]()
    graph[a].incl(b)
    graph[b].incl(a)

var groups: HashSet[(string, string, string)]
for a in graph.keys:
    for b in graph[a]:
        for c in graph[b]:
            if c in graph[a]:
                let group = sorted(@[a, b, c])
                groups.incl((group[0], group[1], group[2]))

echo groups.toSeq.filter(group => @[group[0], group[1], group[2]].any(node => node.startsWith("t"))).len
