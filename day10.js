const partOne = lengths => {
  const hash = knotHash(256, lengths);
  return hash[0] * hash[1];
};

const partTwo = lengths => hexString(xorChunks(knotHash(256, convertLengths(lengths), 64)));

const knotHash = (listSize, lengths, rounds = 1) => {
  let list = [];
  for (let i = 0; i < listSize; i++) {
    list.push(i);
  }

  let position = 0;
  let skipSize = 0;

  for (let i = 0; i < rounds; i++) {
    for (let length of lengths) {
      reverse(list, position, length);
      position += length + skipSize++;
    }
  }

  return list;
};

const reverse = (list, from, length) => {
  for (let i = 0; i < Math.ceil(length/2); i++) {
    const a = (from + i) % list.length;
    const b = (from + length - i - 1) % list.length;
    const temp = list[a];
    list[a] = list[b];
    list[b] = temp;
  }

  return list;
};

const convertLengths = lengths => {
  const lengthsString = lengths.toString();
  const result = [];

  for (let i = 0; i < lengthsString.length; i++) {
    result.push(lengthsString.charCodeAt(i));
  }

  return result.concat([17, 31, 73, 47, 23]);
};

const xorChunks = input => {
  const chunks = [];

  for (let i = 0; i < 16; i++) {
    let chunk = input[16 * i];
    for (let j = 1; j < 16; j++) {
      chunk ^= input[16 * i + j];
    }
    chunks.push(chunk);
  }

  return chunks;
};

const hexString = input =>
  input
    .map(n => ('0' + n.toString(16)).slice(-2))
    .join('');

const lengths = [189, 1, 111, 246, 254, 2, 0, 120, 215, 93, 255, 50, 84, 15, 94, 62];
console.log(partOne(lengths));
console.log(partTwo(lengths));
