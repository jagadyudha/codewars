// Find The Parity Outlier

// Examples
// [2, 4, 0, 100, 4, 11, 2602, 36]
// Should return: 11 (the only odd number)

// [160, 3, 1719, 19, 11, 13, -21]
// Should return: 160 (the only even number)

int find(List integers) {
  List<int> odd = [];
  List<int> even = [];

  //check even or odd
  for (final i in integers){
    if (i % 2 == 0){
      even.add(i);
    }else{
      odd.add(i);
    }
  }

  // check if even > 1
  if (even.length == 1){
    return even[0];
  }else{
    return odd[0];
  }

}

void main(){
  print(find([160, 3, 1719, 19, 11, 13, -21]));
}