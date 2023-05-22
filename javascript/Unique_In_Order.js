// Implement the function unique_in_order which takes as argument a sequence and returns a list of items without any elements with the same value next to each other and preserving the original order of elements.

// For example:

// uniqueInOrder('AAAABBBCCDAABBB') == ['A', 'B', 'C', 'D', 'A', 'B']
// uniqueInOrder('ABBCcAD')         == ['A', 'B', 'C', 'c', 'A', 'D']
// uniqueInOrder([1,2,2,3,3])       == [1,2,3]

var uniqueInOrder = function (iterable) {
  const tempArr = [];
  const arrCheck = Array.isArray(iterable) ? iterable : iterable.split("");

  for (let i = 0; i < arrCheck.length; i++) {
    if (arrCheck[i] != arrCheck[i + 1]) {
      tempArr.push(arrCheck[i]);
    }
  }

  return tempArr;
};

console.log(uniqueInOrder("AAAABBBCCDAABBB"));
