/**
 * Had to resort to JS to not rip my hair out trying to get explode to work
 * properly. Will try to rewrite in Rust later, maybe.
 */

function addAllNums(nums) {
    let num = nums[0];
    for (let i = 1; i < nums.length; i++) {
        num = addNums(num, nums[i]);
    }
    return num;
}

function addNums(num1, num2) {
    const num = {
        type: 'pair',
        children: [num1, num2]
    };
    num1.par = num;
    num2.par = num;
    reduce(num);
    return num;
}

function reduce(num) {
    while (true) {
        if (explode(num, 0)) continue;
        if (!split(num)) break;
    }
}

function explode(num, depth) {
    if (num.type === 'pair' && depth > 3) {
        const left = find(num, 0);
        if (left) add(left, num.children[0].val, 1);
        const right = find(num, 1);
        if (right) add(right, num.children[1].val, 0);
        num.type = 'reg';
        num.val = 0;
        return true;
    } else if (num.type === 'pair') {
        if (explode(num.children[0], depth + 1)) return true;
        return explode(num.children[1], depth + 1);
    }
    return false;
}

function split(num) {
    if (num.type === 'reg' && num.val > 9) {
        num.type = 'pair';
        num.children = [{
            type: 'reg',
            val: Math.floor(num.val / 2),
            par: num
        }, {
            type: 'reg',
            val: Math.ceil(num.val / 2),
            par: num
        }];
        return true;
    } else if (num.type === 'pair') {
        if (split(num.children[0])) return true;
        return split(num.children[1]);
    }
    return false;
}

function find(num, i) {
    if (!num.par) return null;
    if (num.par.children[i] !== num) return num.par.children[i];
    return find(num.par, i);
}

function add(num, n, i) {
    if (num.type === 'reg') {
        num.val += n;
    } else {
        add(num.children[i], n, i);
    }
}

function parseNum(input) {
    return parseNum2(input, 0)[0];
}

function parseNum2(input, i, par) {
    if (input[i] === '[') {
        const me = { type: 'pair', par };
        const [a, j] = parseNum2(input, i + 1, me);
        const [b, k] = parseNum2(input, j + 1, me);
        me.children = [a, b];
        return [me, k + 1];
    } else {
        let val = Number(input[i]);
        let j = i + 1;
        while (Number(input[j])) {
            val = val * 10 + Number(input[j]);
            j += 1;
        }
        return [{ type: 'reg', val, par }, j];
    }
}

function magnitude(num) {
    if (num.type === 'reg') return num.val;
    return 3 * magnitude(num.children[0]) + 2 * magnitude(num.children[1]);
}

const input = [
    '[[[5,[4,8]],0],[[[2,2],5],[7,9]]]',
    '[[[[4,2],0],[4,[9,9]]],[[[7,0],[9,8]],5]]',
    '[[[[3,2],2],4],0]',
    '[[8,7],[[9,4],[8,[5,5]]]]',
    '[[[1,7],[[8,2],[3,5]]],[6,8]]',
    '[[[[7,1],[4,2]],[[6,0],[3,8]]],[[[5,2],8],[7,[4,7]]]]',
    '[[6,[5,8]],[[4,[3,0]],5]]',
    '[[[[1,2],[1,5]],[[7,1],6]],[2,[4,7]]]',
    '[[9,[[5,3],3]],9]',
    '[6,[[[6,1],2],[6,[5,6]]]]',
    '[[[9,8],[[5,6],7]],[[4,[9,9]],[8,1]]]',
    '[[4,7],[[[3,1],[1,5]],[5,8]]]',
    '[[[9,7],[5,[6,0]]],[[9,[3,5]],7]]',
    '[[[[1,2],[1,4]],4],0]',
    '[[[[0,0],[1,2]],[[0,2],[1,6]]],[0,[[6,2],[5,1]]]]',
    '[[[1,2],[[0,2],4]],[[5,[7,3]],2]]',
    '[[5,[1,[6,3]]],[5,[2,[5,3]]]]',
    '[[[9,[7,7]],7],[[8,1],[[9,1],7]]]',
    '[[[[6,5],6],[6,5]],[7,[9,[3,9]]]]',
    '[[6,[9,3]],6]',
    '[[5,[[2,3],[9,1]]],[0,[[5,8],4]]]',
    '[[[[4,9],[2,3]],7],6]',
    '[[[2,6],6],[[[9,0],9],[4,[6,1]]]]',
    '[[[9,[9,1]],[4,4]],[0,[6,8]]]',
    '[8,[2,[[0,4],[5,4]]]]',
    '[[3,[9,4]],[[0,[6,9]],2]]',
    '[[[1,1],[[0,1],[1,9]]],[[5,4],[6,9]]]',
    '[4,[2,[[6,9],0]]]',
    '[[[6,[3,7]],[3,7]],[1,[2,[4,7]]]]',
    '[[[[6,4],[0,0]],[[8,2],5]],[[8,[2,4]],[4,[9,1]]]]',
    '[[[[8,1],[8,0]],[5,[7,6]]],[2,[[0,2],[9,2]]]]',
    '[[6,7],[[9,[1,1]],[[9,2],9]]]',
    '[[[[7,2],[8,8]],0],4]',
    '[[[2,1],[[3,1],9]],9]',
    '[[[[5,5],9],[[7,8],[6,0]]],[[[4,0],[0,6]],[6,2]]]',
    '[[6,[3,[9,4]]],[[[5,5],5],2]]',
    '[[[4,3],[9,[8,4]]],4]',
    '[[[0,[5,9]],[[9,6],8]],[7,[3,[8,9]]]]',
    '[[6,[[8,2],[0,2]]],[[8,8],[[7,9],2]]]',
    '[[[0,[8,0]],7],[[[7,2],[6,6]],[[5,5],5]]]',
    '[5,[[1,[3,6]],[[0,7],6]]]',
    '[0,[[[5,7],[6,2]],8]]',
    '[[[4,[5,4]],[[2,9],[5,3]]],[7,[2,4]]]',
    '[[6,[[8,4],6]],9]',
    '[[[7,[7,7]],[2,9]],[8,[5,[6,4]]]]',
    '[[[[7,9],[9,9]],[[6,1],[5,5]]],[[[4,3],[7,3]],[6,[0,3]]]]',
    '[[2,[2,0]],6]',
    '[[[[2,3],2],1],[0,2]]',
    '[[[[8,6],[5,6]],3],1]',
    '[[[[4,9],[2,4]],2],[2,[[6,3],[3,4]]]]',
    '[0,[[[1,0],[4,0]],8]]',
    '[[4,[6,[2,1]]],[[[5,8],4],[[8,0],4]]]',
    '[[[0,0],[[3,4],1]],[9,[1,[7,0]]]]',
    '[[0,0],[[[9,3],8],[[1,7],[4,6]]]]',
    '[[[4,3],3],[[[3,3],9],9]]',
    '[[[[2,0],[0,1]],[[1,2],[1,0]]],[[[6,6],1],[7,1]]]',
    '[[1,[[2,7],9]],[[[9,1],6],[[7,0],0]]]',
    '[[7,[[5,4],0]],8]',
    '[[6,9],[[[8,1],6],[5,[1,2]]]]',
    '[[7,6],[[[1,9],2],[0,3]]]',
    '[[[9,7],[9,[5,2]]],[[[0,0],2],[0,8]]]',
    '[[9,[6,2]],[5,8]]',
    '[[[6,[0,3]],[[5,1],[4,4]]],[6,[5,[1,9]]]]',
    '[[8,8],[[[3,1],7],[[8,3],3]]]',
    '[[[[1,1],[9,5]],9],[[[2,8],[6,4]],[[1,2],[4,5]]]]',
    '[[[1,7],8],[[5,[0,6]],[9,[3,3]]]]',
    '[[7,3],[[[8,2],3],4]]',
    '[[9,3],[[1,[7,0]],5]]',
    '[[[9,[2,2]],[7,5]],[[7,[1,7]],[[0,5],7]]]',
    '[[1,[[0,3],3]],1]',
    '[[9,[[3,0],[9,0]]],1]',
    '[[2,[[3,9],7]],[[[8,1],[7,2]],[9,[6,3]]]]',
    '[4,[[0,[0,4]],[0,1]]]',
    '[[[[2,8],6],[[6,6],[5,8]]],[[1,[7,5]],[[2,2],[6,0]]]]',
    '[[[6,7],8],[[[1,5],[9,3]],[0,2]]]',
    '[[[[6,6],[6,2]],[0,6]],[[[1,5],2],[[0,3],[3,9]]]]',
    '[[5,[8,2]],[3,8]]',
    '[[8,7],[[0,5],[3,[6,8]]]]',
    '[[6,[[2,3],5]],[[9,[0,8]],[[2,4],[1,8]]]]',
    '[[[[5,7],[4,3]],[[5,4],5]],[0,[[6,5],2]]]',
    '[2,[[5,[0,7]],[3,[4,0]]]]',
    '[1,9]',
    '[[[[1,4],1],[0,[1,2]]],2]',
    '[[4,3],[5,[6,4]]]',
    '[[[4,4],[[8,0],[6,5]]],[[4,[9,1]],[[1,1],[2,2]]]]',
    '[[4,3],[[[1,1],1],[[4,6],[5,7]]]]',
    '[[[[6,1],[5,3]],2],[[[0,6],[7,3]],8]]',
    '[[[2,8],5],[1,[3,[8,7]]]]',
    '[[7,[5,[9,0]]],[[[9,1],2],[2,[9,6]]]]',
    '[[[[7,3],1],[[4,6],[5,1]]],[[[4,7],4],[[5,2],[3,7]]]]',
    '[[[[2,3],8],[7,8]],[[[5,5],[2,5]],[[6,8],1]]]',
    '[[[2,1],[[8,9],[4,3]]],[[8,[9,0]],7]]',
    '[[[[8,2],5],0],[[8,[9,6]],[[6,1],1]]]',
    '[[[3,[4,9]],[[5,4],[2,2]]],4]',
    '[[[[9,8],4],[[7,4],9]],[[0,7],6]]',
    '[[7,[[6,1],8]],[[2,0],[2,5]]]',
    '[[[[3,2],6],5],6]',
    '[6,[3,5]]',
    '[[[[7,1],7],4],[[[4,6],5],[1,[7,9]]]]',
    '[[[[7,0],7],[8,9]],[5,[[2,5],6]]]'
];

console.log('Part one:', magnitude(addAllNums(input.map(s => parseNum(s)))));

let max = 0;
for (let i = 0; i < input.length; i++) {
    for (let j = 0; j < input.length; j++) {
        if (i === j) continue;
        const num = addNums(parseNum(input[i]), parseNum(input[j]));
        const mag = magnitude(num);
        if (mag > max) max = mag;
    }
}
console.log('Part two:', max);
