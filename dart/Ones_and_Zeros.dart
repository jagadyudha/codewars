// Ones and Zeros

// Examples:
// Testing: [0, 0, 0, 1] ==> 1
// Testing: [0, 0, 1, 0] ==> 2
// Testing: [0, 1, 0, 1] ==> 5
// Testing: [1, 0, 0, 1] ==> 9
// Testing: [0, 0, 1, 0] ==> 2
// Testing: [0, 1, 1, 0] ==> 6
// Testing: [1, 1, 1, 1] ==> 15
// Testing: [1, 0, 1, 1] ==> 11

import 'dart:math';

int binaryArrayToNumber(List<int> arr) {
  var result = 0;
  //looping with reverse
  for (int i = arr.length-1; i >= 0; i--){
    //reverse array
    var arr_reverse = arr.reversed.toList();
    
    //get Exponentiation every looping
    result = result +arr_reverse[i]*(pow(2, i).toInt());
  }
  return result;
}

void main(){
  print(binaryArrayToNumber([1, 1, 1, 1]));
}