// Count the number of Duplicates

// Example
// "abcde" -> 0 # no characters repeats more than once
// "aabbcde" -> 2 # 'a' and 'b'
// "aabBcde" -> 2 # 'a' occurs twice and 'b' twice (`b` and `B`)
// "indivisibility" -> 1 # 'i' occurs six times
// "Indivisibilities" -> 2 # 'i' occurs seven times and 's' occurs twice
// "aA11" -> 2 # 'a' and '1'
// "ABBA" -> 2 # 'A' and 'B' each occur twice

int duplicateCount(String text){
  int result = 0;
  var current = null;

  //convert to lowecase
  var text_lower = text.toLowerCase();
  // split to array
  List<String> split_to_array = text_lower.split('');

  //sort array
  split_to_array.sort();

  // check every data inside array
  for( int i = 0 ; i < split_to_array.length-1; i++ ) {
    if (split_to_array[i].toLowerCase() == split_to_array[i+1].toLowerCase()){
      if(current != split_to_array[i+1].toLowerCase()){
        current = split_to_array[i+1].toLowerCase();
        result++;
      }
    }
   } 

  return result;
}

void main(){
  print(duplicateCount('jagadyudha'));
}