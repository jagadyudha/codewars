'''
An isogram is a word that has no repeating letters, consecutive or non-consecutive. 
Implement a function that determines whether a string that contains only letters is an isogram. 
Assume the empty string is an isogram. Ignore letter case.

Example:
"Dermatoglyphics" --> true
"aba" --> false
"moOse" --> false (ignore letter case)'''

def is_isogram(string):
    string = string.lower()
    string = list(string)
    result = True
    string.sort()

    for i in range(0, len(string)-1):
        if (string[i] == string[i+1] and string[i] != '' and result == True):
            result=False
    

    return result
    

print(is_isogram('moOse'))