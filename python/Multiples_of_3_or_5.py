# Multiples of 3 or 5

# Finish the solution so that it returns the sum of all the multiples of 3 or 5 below the number passed in. Additionally, if the number is negative, return 0 (for languages that do have them).
def solution(number):
    result = 0
    for i in range (1, number):
        if (i % 3 == 0) or (i % 5 == 0):
            result= result+i
    return result
print(solution(10))