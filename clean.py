import re


def clean_text(file_path, output_path):
    """
    Cleans the text file to remove markers, TOC, and chapter titles, leaving only narrative content.

    Parameters:
        file_path (str): Path to the input text file.
        output_path (str): Path to save the cleaned output.
    """
    with open(file_path, 'r', encoding='utf-8') as file:
        text = file.read()

    # Remove Gutenberg license and meta sections
    text = re.sub(r"\*\*\* START OF.*?\*\*\*", "", text, flags=re.DOTALL)
    text = re.sub(r"\*\*\* END OF.*?\*\*\*", "", text, flags=re.DOTALL)

    # Remove TOC and illustrations
    text = re.sub(r"CONTENTS\..*?(\n\n|$)", "", text, flags=re.DOTALL)
    text = re.sub(r"ILLUSTRATIONS\..*?(\n\n|$)", "", text, flags=re.DOTALL)
    text = re.sub(r"\[Illustration:.*?\]", "", text, flags=re.DOTALL)

    # Remove chapter titles
    text = re.sub(r"_CHAPTER .*?_(.*?)\n", "", text)

    # Remove excess newlines and normalize spacing
    text = re.sub(r"\n{2,}", "\n\n", text)
    text = text.strip()

    # Save the cleaned text
    with open(output_path, 'w', encoding='utf-8') as output_file:
        output_file.write(text)


# Paths for the input and output files
input_path = 'C:/Users/lucab/Downloads/pg1342.txt'  # Replace with the uploaded file path
output_path = 'data.txt'  # Path to save the cleaned output

# Run the cleaning function
clean_text(input_path, output_path)

print(f"Cleaned text saved to {output_path}")
