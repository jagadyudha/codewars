'''
Sort the odd
You will be given an array of numbers. 
You have to sort the odd numbers in ascending order while leaving the even numbers at their original positions.

Examples
[7, 1]  =>  [1, 7]
[5, 8, 6, 3, 4]  =>  [3, 8, 6, 5, 4]
[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]  =>  [1, 8, 3, 6, 5, 4, 7, 2, 9, 0]
'''

def sort_array(source_array):
    oddarr = []
    for i in range(0, len(source_array)):
        if (source_array[i] % 2 != 0):
            oddarr.append(source_array[i])
    oddarr.sort()

    for index, item in enumerate(source_array):
        if (item % 2 == 0):
            oddarr.insert(index, item)
    
    return oddarr

print(sort_array([9, 8, 7, 6, 5, 4, 3, 2, 1, 0]))