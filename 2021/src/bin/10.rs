use std::collections::HashMap;

static INPUT: &str = "<{({(([[<<([[([][])]{([]<>)([]<>)}]<[[[]{}]{()[]}]>)(<<[[]()][[]()]><[<>[]]<[]()>>>{<<<><>><<>[]>>[<[]<>><\n{<[<((<<({([{{{}[]}<<><>>}]<<{[]<>}([]<>)>>)<{(<[]()>{()()})<(()<>){(){}}>}([(()[])<[]>][[[][]]{{}}])\n<{[[{<[[{(<[[({}())[()()]]([<>()](<>()))][(([]<>)([][]))(({}{})[<><>])]><<<[[]<>][{}[]]>([<>{}\n<[[<[([({[{<[({}<>)([]<>)](<<>[]>([]()))><[<[]{}>[<><>]]>}>[{[[({}<>)<()<>>][{{}[]}(()())]][[(<>{}){<>{\n[((<{((([[<<<[<><>]{{}[]}>>(([<><>]<()[]>)<(()<>)>)>([[[{}{}][[][]>]<{<>()}[<>{}]>])]]<{[<[\n([{([[[{(<({<<[]<>>[()()]>{(()[])}})<(<<{}()>(<>>>([(){}]))<[(<>{}){()}]((()[]){()()})>>>)}{\n<{{[(<[<<[[{<(()[])>[[{}()]{{}[]}]}[[(<>())[()()]]{<{}{}>}]](<{[<><>]{{}{}}}<({}[])[<><>]>><{[{}<>](<><>)\n{({<[<([[(({({<>()}{{}{}})})<<<((){})[[]()]><{{}()}[[][]]>>{(<{}{}>{{}()})}>)<<{<[{}[]][{}{}]>([[][]]{\n{[(((((<{[({<{()[]}(()())>{<(){}>{{}()}}}(<[<>()]{[]()}>(<()()>{{}<>})))[<[[{}{}](<>()>]({[]<>\n(([((<{<[((<{{<><>}{[]{}}}><<<<>>[{}]>([[]{}]<<>[]>)>)[[{<{}>{()()}}[[{}[]](()()>]](<<<>{}>[[]\n{<([[{<{([[<([[]<>]({}{})){{{}()}({}[])}>](<(<()()><()[]>)<<[]<>>([]{})>>)]){[{<<[()[]]<<>{}>><({\n<{[[([<<<(<{{(()<>)(()())}([{}<>]((){}))}>)>({({{{()<>}({}<>)}<[[]<>]{<>[]}>})(({<(){}>[<>[]]}<([]{})<[]{}>>)\n(<{<{[[({(<[((<>[]){{}})<(<><>)<[]{}>>]((<<>[]><[]()>)<<{}{}>{(){}}>)>(<[[[]<>]]{{<>{}}{[]()}}>(<{\n{(({{[[{<[((<<()<>>[{}{}]>((()<>){[]()})){{(<>{})[<>[]]}<<<><>>>}){{({<>{}}([]()))[{<><>}{[]\n(([<(([({{<{[{()<>}<{}[]>]}{[[()()][()<>]}[{<>{}}({}{})]}>((<<<>{}>[<>()]>{{[]<>}({}{})}){[<{\n[[{{(<<[[[[<<<<>{}>({}())>[([]<>)(()())]>]<<<{<>()}>[<<>[]>[()()]]>[([[]()]<[][]))<[<>{}]{{}<>\n{[{(((([<[{([(()()){()()}][<(){}>[{}<>)])[(<{}<>>{<>()})({{}<>}[()[]])]}(<({[]<>}<()[]>)({[]<>\n<<([(([{<{<<<[{}[]]((){})>((<>[])(()<>))>{{{<>()}{[]}}{([][])}}>[<{{<>[]}<<>>}({[]<>})>]]({({{<><>\n[{((<{<[[<[{[{{}<>}]{[{}][<>[]]}}[<([]())<()[]>>((<>){[]{}})]]<{(<()<>><{}<>]){[[]<>]({}{})}}({<{}[]>{{}\n<[<{(<[[<(((<({}[])><(<><>)(<>())]){{{()()}(<>())}<[{}()]>})[([[<>[]]{{}[]}]{{{}()}<{}[]>})[[(()<>){<>\n[([{{[<([{[([[()<>]])([[<>][()[]]][[<>[]]<<>[]>])]}{([<<{}()>({})>(<[]()>(<><>))]<<{{}<>}{<>}><(<>\n<<[[([([(<(<<[{}[]]<()[]]><{[]{}}[<>()]>>)>[<<({[][]}<<><>>)[<{}{}>{()<>}]><({<>{}}([]))((\n<({<{(<[({([[{()}(<><>)](<[][]>[{}{}])](([[]<>]<<>{}>))){({<<>{}>[[]{}]}<([][])>)<<(<><>)<[]<>>}>}}((([[()<\n{([<(<[(({[({[(){}]<<>{}>}[[{}{}][{}[]]])]{({[()<>]{()[]}}(<[][]>[()[]])){{{[]{}}(<>[])}[<(){}>(\n<[<[<{[{<[{[<<{}<>>(<>)>]{<({}{}){(){}}>((<>{})[{}<>])}}[<[[<><>]{{}<>}]{<()()>}>{<<[]()>(()())>{{{}<\n(({[[<<[({[{{<()()>}}{<[<>()]>}][(<{[][]}>{[{}()]{[]<>}})<{{<>{}}(()[])}{{[]()}<<>[]>}>]})\n<((<([[({(<{[[(){}]<[][]>]}>({<[()<>](()[])>(([]<>)([]<>))}<{[[]<>]{(){}}}(<()<>>([][]])>))}<{[<[({}\n{({{{[[{{[{(<<(){}>[{}{}]>((()<>)[<>{}]))[<<{}[]><[]>>[{{}{}}(<>[])]]}[(<<(){}>({}{}))((<>{})([]<>)))\n[[(({[{(<({[{<[]()>{()[]}}<(<>())<()()>>]<(({}[]){{}<>])((()<>){{}[]})>}([[({}{})[<>()]][{<><>}]]{([[\n[[{<[([<[((<([[]{}][{}<>])([{}<>])>[([()<>]{{}{}}){{[]()}}])<({{[]()}[[]()]))>)([<<<<>{}><{}()>><(<>\n<<({{{<{[<{<{<[]<>>[[]{}]}[<[]()>{[]{}}]>[[[(){}][<><>]]{(<>[]}({}{})}]}><[[({{}[]}{[]()}){(<>())[()<\n{{(({[[[{<[<(([]())([]{}))([{}{}](()[])}>]>}([<{[[[]()][(){}]]<{<>[]}>}({<()<>><[]<>>})>((([()\n(<(({{(([(({{<{}[]>({}())}([<>[]][{}()])}((<[][]>{()<>}){[[]<>]<{}{}>}))<({{[]<>}([][])}{<[]{}>{<>[]}})\n((([<([([{{{<<<>{}><{}[]>>[[<>[]][{}[]]]}}(([<<>{}>[()[]]][{{}{}}])(<[()()][<><>]>{{<>[]}})}}]<{[[<\n{{(<[<[<[<<[[(<>[])]({<><>}[()()])]>>]>[{{[[{(<>[])([][])}{[[][]]([]())}]>[[[<{}{}>({}())]<{<>\n<[{[[([[{[(<[<{}{}>{(){}}][{[]<>}<()[]]]>(([()<>][[][]]){[<>{}](()[])}))[[<<{}>[<>{}]>[({}{})({}{})]]<([[]{}\n({<<<{<({{{{{[()()]({}())}[[[]{}][<><>])}({<()()><[]()>}({<>()}))}{(((<>()){()()}){<(){}><()[]>})(([{}()]\n([<({[{({(<<{[{}()]}{{{}[]}((){})}>>)<<{[((){})[()]]}<<{[]<>}<<>()>>([<>()][[]<>])>>{<<[<>[]]<[]<>>)<{<\n(<[<<(<[[[[{{({}<>)[{}{}]}<[[]{}]([]<>)>}][<[<<>{}>{[]}][[[]{}](<>())]><<[<>()][(){}]><{[]\n(({[[(([<(([{({}[])[<>{}]}{(<>[])({}{})})<<<{}[]>({}())>[({}<>)<{}()>]>)[<<[[][]]((){})><{{}(\n{[((<([<[{[{<[<><>][()()]><{<>{}}>}[{<[][]>[()<>]}<(<>())<()<>>>]]<<<(<>())<()[]>>>[{{{}[]}}[[{}()]((){\n{{<{({{({[<((({}{}){<>[]}){<{}()>{<>[]}})>(<{<<>{}><[]<>>}>[[{[]<>>{<><>}]{((){})<(){}>}])]}(<{[{{<><>}{[\n(({<[[{[(<(((({}{}){()<>}))<{{[]{}}{{}{}}}<[{}<>][(){}]>>)>[{{(<<><>>([]{}))[((){})<{}[]>]}}(([[<>()]{<>()}]\n[<([[[<[<{<{<{[]{}}{[]{}}>[([]())<<>[]>]}({{[]{}}<[]()>}<({}())(<>{})>)>}[({[[{}[]][{}<>]]})]>]><[\n<{[<<[[<{[[{<<{}[]>[<><>]>}({[[]{}]<{}()>}((()[])))]{[<{()}[[]{}]>]{(({}[])<{}()>)<(<>())[()<>]\n[[([[{[[(([{[[<>()]{<>()}]({(){}}({}{}))}[<[<><>]([]())>(<[]()><{}[]>)]]{(<<<><>>[[]{}]>(<[][]>((){}))\n{(<<[[{[{{(<<<{}[]>(<>[])>({[]{}}([]{}))>[{<<>()>[[]<>]]([{}[]])])}[[{(<<><>>[()()])<<<><>>>}([<<>\n([((<(<[(<{((<{}{}>{<><>})){{(()[])[{}[]]}}}{[[[(){}][<><>]]]({<()<>>{{}[]}}<[()()]{()[]}>)}>[<{(<{}\n[(<[([[({({<[{<>()}([]<>)][(<><>)<()[]>]>})[{([(())({}[])][{[]<>}<()<>>])}({<[{}{}](()())><({}())[<>[]\n([(<<[<{<{((<(<>[])({}<>)>{{()()}<<>{}>}))<{[[()[]][()[]]][({}[])<{}()>]}{<({})><{[][]}{{}<>}>}>}<<<({(){}}\n[(<{(([({<{{(<[]<>>({}()))}}[<[[{}[]][[]]]>[({[]<>}(<>{}))]]>})]<([(<([[()()][[][]]]<((){}\n((({([(<<{{<<({}())(())>([<>()]<()()>)>}}<[(<{{}[]}<()<>>><({}{}){(){}}>)][(({()<>}([]()))([()]))({{{}()}\n{<(({[<([{[({([][])[[]{}]}<({}<>)<<>()>>)<<{<>()}{[]<>}>{{{}}{[]()}}>]}])<<{<(([[]{}]<<>[]>)){<<[]<>>><<()[\n[{<(<<<((<<<{[()<>]{[]<>}}>[[<{}{}>[()[]]]<(<><>)[()[]]>]>{[[[[]()][<>[]]]]({(<>}{[]{}}})}>){({\n<<[((<<[{{[(<[<>()]><{{}[]}{<>{}}>){[(()[])([]())][[(){}][{}[]]]}]([<[[][]]{(){}}><<{}{}>{<>{}}>][[\n{<<{<<{<({[[([<>{}])[{[]<>}{{}[]}]][[<()<>>[<>{}]]]]<([{[]()}({}{})]({[]()}{<>()})){[({}<>)[[]{}\n{<<[{((([<([[<[]()><<><>>][{()<>}<[][]>]]{<[{}[]][()[]]>[{<>{}>{[]}]}){[([[]{}]<()[]>)[[<>()][{}[]]\n({{{{<([[((({{{}{}}<()[]>}{<()<>>{{}<>}})(<({}{})>({<>{}}[[]()]))){(<<<><>><()[]>>[{()()}{[]<>}\n({<{[[{({{[<[{[][]}<[][]>]{(<><>)}>([[<>{}][<>{}]]({[]{}}<<>[]>))]}{[({[[]]{()[]}}{(<>{})[()()]})(<{(\n[{<<<[(<(<{((([]{}))[{<>()}[()[]]]){{<{}{}>}[<[]<>>{<>{}}]}}<(<[<>[]][()()]>)<([{}()]{{}[]})\n{{(([[([((<([[<><>]])>({[{()<>}<[]{}>]}<{[()()]<{}>}(<{}<>>({}{}))>))[{{([()()]<{}<>>){(<>{})(\n([(<{[[{<{{<{([]())(<>[])}{{[]<>}{()<>}}>{{{()}}[<[]{}><{}<>>]}}<{[{()()}]{{<>[]}[<><>]}}<<{<>(\n([[{[{[{<<(<{<{}{}>{[]}}{[{}{}]{()[]}}>[{({}[])}<[()[]]>])>>[(<<({<>{}}{[]{}})>>(<[([]()){()<>}]((()())([]\n(({<({<([(({<<<>()>([]())>{<()[]>{[]<>}}}(<{{}<>}[()()]>({{}<>}[{}()]))))]){{<[{[(<>[]){()\n([[(<{[[{<[<(<[]()>[{}[]]){[<><>]{<>[]}}>{{([]()){[]{}}}<((){})([]<>)>}]<<<{{}[]}[()]>(<{}><[][]>)>>>}\n{{(<[{[([<<<[<{}<>>(<>{})]<({}[])[<>[]]>>({<{}{}><[]{}>}<<[]><(){}>>)>>(((({()()}[[]{}]))<[{[]<>)<[]()>]\n(({{<<[{([[({[[]{}]{()<>}}[(<><>)[()()]])[<[<>[]]><([]{})[[][]]>]](<((<><>)<<>()>)[([][])((){})]>[(\n((<<([[<{[{({<(){}><[][]>}{(()[])({}[])})({[{}()]({}{})}(<(){}>(<><>)))}{{[{()<>}<{}{}>][{[]{}}(()\n<{(((([<[((({{{}<>}[[]<>]}<[{}<>]>)[<({}{})>{{[]()}<{}()>}])<([<<>>(<><>)]((()())(<>{})))>){(({(\n<{{<<{{{[({{[<()[]>(()[])]}<<[[]<>][[]]>>}[<[({})<<>[]>][[[]]([][])]>])]}}(<({[[<[(){}]>{<[]{}>}]{(({\n[([[(({([{{({{()}[{}<>]}{({}<>){{}()}}]}}{[<<({}())<(){}>>[{()[]}([]{})]>]{<<{{}()}(<>[])>{[(\n[{[{[([<<((<<{()<>}<<>()>>[{[]{}}{{}[]}]>[[{[][]}{{}}]])(([((){})(()())]{[(){}]{()}})))}{(<<<<<><>>({}[]\n<[{({[({[({{(<[]()>{{}<>}){{{}<>}[(){}]}}(([{}()]<[][]>)[[<>]({}{})])}<<[(<>)({})]<<()[]><\n[((([<(({<<({{{}()}({}<>)}({()[]}[{}{}]))>{[{(<>[])}{(<>[])<{}[]>}]{<[<><>][()[]]>}}>({<{<<>()><<><>>}([{}[]]\n{({((<<([((<[[()[]]{[][]}]<{(){}}{<>{}}>>({[(){}][<>]})))(<(({<>[]}<(){}>)(<()[]><{}()>))>)]){\n<{<[<(<{<[({{{{}<>}<<>{}>}{[[]()][{}{}]}}[({<><>}{<>{}})[[{}()]{[]()}]])({{<[])(<>[])}})](<<[[<><>]](<{}{}>\n[([[[{[((<<(<<()()>>)><<{([]()){<>{}}}<{[]{}}<()()>>>[(<<><>>[()[]]){((){})([]{})}]>>({[<[<><>]{<><\n<{{({{{[<{[{{{()<>}[<><>]}[(()[]){[]{}}]}(<{{}[]}[<>[]]><{{}{}}<{}[]>>)]{(({(){}}<<>{}>)[<<><>>[{}\n[[([[{[({[{[({{}[]}{()[]})[<()()>{{}{}}]][(([]<>)<[][]>)[<{}{}><{}()>]]}][<([([][]){()()}][[{}{}]{()<\n{(<<(((<((((<{()<>}<<>[]>)[<[][]>[{}()]]){[[[]<>]<(){}>][<()<>>{{}<>}]})))<[{{[{{}{}}<[]<>>]}}(\n{[(<{[<{{({[{[<><>](<>{})}{({}[])(<>{})}]}({<({}[])<[]<>>>(({}[]))}[<<()><<>>}[{()[]}{[][]}]])){[({<<>()>(\n<([{[(<(<<([((()())<{}{}>)]<{(<>())(<>()]}{[()<>](<>())}>)<[[<[][]>]{{()}<<><>>}](({()()}[[\n<[<[<{{{[{((<(()()){{}<>)><[[]()](()<>)>))({[[<>{}]{{}()}]({()<>}(()[]))})}]}([{((({{}()}[()<\n[{([({[<(<{[{[<>[]][<>()]}([[]{}](()()))){{[()()]<(){}>}({()[]}<[]()>)}}<{[(<>[]){()<>}][[()<>]{()[]}]}{<(<>\n{((<(<({{([{[([]<>){[]<>}](<[]<>>[[]<>])}{{<[][]><()[]>}({()<>}<<>{}>)}]<[<[<>()]([]{})><{{}<>}<{}<>>>][(\n{<<([<(<{[[{{{<>[]}[[]<>]}{[{}[]]]}{<[(){}]>{[{}()]{[]<>}}}](<{[[]<>]{<>[]}}<<()<>>[[]()]>>)]<[{{<{}>[{}]}<(\n[[{{{<{[{({<<<[]{}>[()<>]><[[]<>]([]<>)>><{<<>()>[<>{}]}>}((<[{}{}]{[]}]{<[]<>>{<>{}}})[([<>(\n[[{<<{{[(([[[{()[]}][[()[]]<[]>]]({{{}}})][({([][])[{}{}]}<[(){}]<()[]>>){<{[]{}}[(){}]><{{}{}}<<>()>]}]))<<\n[(({[<<{<({<([<><>])[{<>()>{{}[]}]>((<()<>><{}<>>)[{[][]}([]{})])})<{{([[]<>]<{}<>>)([()[]]{()()}\n[([(({(<[<[<((<>{})[[]<>]){[<><>]({}[]>}>]>]>)}[(({(<<[{(){}}[()[]]]>[<(<>{}){{}{}}>[[()<>][()]]]>)\n{<{{<({<{((<{({})<<>{}>}((()){()<>})>))}((<{((()[]){{}[]})[<<>>[[]<>]]}(([<>](()<>)))><{{{<>()}((){}}}((\n<[<{<((({{[<{{[][]}}<([]<>)(<>{})>>]}[(([{()()}{<>()}])[<[()[]][()[]]>]){<<[<>()](<>{})>{{(){}}(<>\n(([{([{[<(<(((()<>))[(())(<>{})])([[{}<>]{{}[]}][{<><>}[{}{}]])>[(([()[]]<[][]>)[{{}[]}(()[])])(([{}{}]\n{<(<<{<[{<{<<<<>[]>[[]<>]){[[]()][<><>]}>({(<><>)(()<>)})}>[<([{{}{}}<[]<>>])({{[]<>}({}[])})>]}{<<[(\n<{([({{{<{[{(([]())<<><>>)<[()()]((){})>}]}(<{(<(){}>(<>()))}([[()[]][(){}]](<<>{}>[{}()])}>)>([{<(\n((([{<<[{{[(<<[]<>>(<>())>{({}{})<{}[]>})[[<()()>[<>[]]][<{}[]>(()())]]]}}]>>)]{{{{[[{[[[([]<>){\n({(<{{({[[{<<(()[])[()]>[{<><>}{<>[]}]>{<{<><>}{()[]}>{[{}()]{[][]}}}}{<<{[]<>}(<><>)>{{[]{}}[\n(<[{{(([<{{(<[<>()]((){})>{<{}[]>({}())})}}>][<[{{([{}[]])<([]<>)<<>{}>>}}{<({[][]}(()()))(<<>()>{\n(<<{{{((<{({<<{}()><{}{}>><<()[]>[<>[]]>>(<[(){}]([]<>)>[({}<>)]))(({{[]<>}<()<>>}[<()[]>(()[])]){<[\n<[(((<({([({{[[]()]{[]()}}([{}()]<[]<>>)}[[<()[]>{{}{}}]([<>]<<>()>)]){[((()<>){<>[]}){[{}[]){[\n(({[[({{{[{{({(){}}{{}[]})<({}[])([]<>)>}[[(<>{})]<([]()){{}()}>]}[{<{[]<>}<()()>>([{}<>]<{\n<[({{[<[{({[({[]<>}{{}[]})[{[][]}{[][]}]][[<{}[]>{[]()}]([()[]]({}<>))]}{<{[{}<>]<[]()>}<<[]()>[<>{}]>><((\n[{[{<{{[<<[[<<{}<>>>][{{{}[]}({}())}]]([{<<><>>({}{})}{{[]<>}}]{{{[]()}([]<>)}})><({{<[][]>}}){<{{()><()>}>[<\n{{({([<({(<[({{}{}}<[]{}>)]>[(((<>()){<>()]))[<({}[])(<>{})>{<<><>>{[]()}}]]){{[<(()[])<()<>>>(<()<>>)]([<{}\n[({[<(<<[{<({<{}()>{{}()}})({({})<()()>}[{[]<>}[{}{}]})><[(({}{}))(((){})[[][]])]>}<{([<[]<>>]<[{}[]]((\n[<{{{<<(<[[{<([][]){<>}>}{<[[]]{[]{}}><{<>{}}>}]<<{([]())[<>[]]}><(([]{}){[]<>})[(<>())({}<>}]>>](";

fn part_one(input: &[&str]) -> u64 {
    let points = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let mut sum = 0;
    for line in input {
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '<' => stack.push('>'),
                ')' | ']' | '}' | '>' => {
                    if Some(c) != stack.pop() {
                        sum += points[&c];
                        break;
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    sum
}

fn part_two(input: &[&str]) -> u64 {
    let points = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    let mut scores = Vec::new();
    for line in input {
        let mut corrupted = false;
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '<' => stack.push('>'),
                ')' | ']' | '}' | '>' => {
                    if Some(c) != stack.pop() {
                        corrupted = true;
                        break;
                    }
                }
                _ => unreachable!(),
            }
        }
        if !corrupted {
            let mut score = 0;
            while let Some(c) = stack.pop() {
                score = score * 5 + points[&c];
            }
            scores.push(score);
        }
    }
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn main() {
    let input: Vec<&str> = INPUT.lines().collect();
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}
