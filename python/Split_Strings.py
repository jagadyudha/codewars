'''

Complete the solution so that it splits the string into pairs of two characters. If the string contains an odd number of characters then it should replace the missing second character of the final pair with an underscore ('_').

Examples:

* 'abc' =>  ['ab', 'c_']
* 'abcdef' => ['ab', 'cd', 'ef']

'''

def solution(s):
    toArray = list(s)
    countArray = len(toArray)
    final = []
    if countArray % 2 == 0:
        for i in range(0, len(toArray)-1,2):
            final.append((toArray[i]+toArray[i+1]).lower())
    else:
        toArray.append('_')
        for i in range(0, len(toArray)-1,2):
            final.append((toArray[i]+toArray[i+1]).lower())
    
    return final

print(solution('asdfadsf'))