function day10(listSize, lengths) {
  let list = [];
  for (let i = 0; i < listSize; i++) {
    list.push(i);
  }

  let position = 0;
  let skipSize = 0;

  for (let length of lengths) {
    reverse(list, position, length);
    position += length + skipSize++;
  }

  return list[0] * list[1];
}

function reverse(list, from, length) {
  for (let i = 0; i < Math.ceil(length/2); i++) {
    const a = (from + i) % list.length;
    const b = (from + length - i - 1) % list.length;
    const temp = list[a];
    list[a] = list[b];
    list[b] = temp;
  }
  return list;
}

console.log(day10(256, [189, 1, 111, 246, 254, 2, 0, 120, 215, 93, 255, 50, 84, 15, 94, 62]));