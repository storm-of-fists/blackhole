from pathlib import Path

training_data = Path("projects/dump/eric_project/training.txt").read_text()
decode_data = Path("projects/dump/eric_project/decodethis.txt").read_text()

def get_sorted_alphabet(data_file):
    counts = {}

    for char in data_file:
        if char.isalpha() and char.isascii():
            lower_char = char.lower()
            if not counts.get(lower_char):
                counts[lower_char] = 1
            counts[lower_char] += 1

    return list(dict(sorted(counts.items(),key=lambda x:x[1], reverse=True)).keys())

training_alphabet = get_sorted_alphabet(training_data)
print(training_alphabet)
decode_alphabet = get_sorted_alphabet(decode_data)
print(decode_alphabet)

fully_decoded_data = ""

# for letter in decode_data:
#     next_char = letter
#     if letter.isalpha() and letter.isascii():
#         lower_case_letter =letter.lower()
#         next_char = decode_alphabet[training_alphabet.index(lower_case_letter)]

#     if letter.isupper():
#         next_char = next_char.upper()

#     fully_decoded_data += next_char

# print(fully_decoded_data)