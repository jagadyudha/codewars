function arrayDiff(a, b) {
  const results = [];
  for (let i = 0; i < a.length; i++) {
    if (!b.includes(a[i])) {
      results.push(a[i]);
    }
  }

  return results;
}

console.log(arrayDiff([1, 2, 3, 4], [2, 3]));
