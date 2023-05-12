# eddie
a rudimentary command line text editor, based off the Unix "Ed" editor, written in Rust

Commands:

i - inserts a line above the current line in the buffer\\
a - appends a line below the currentt line in the buffer\\
w - writes the buffer to a text file provided, or creates a text file if one with that name does not exist. returns the file size, in characters\\
p - prints the lines in the buffer\\
h - provides an error message if available\\
n - prints the lines of the buffer, along with their line numbers\\
q - quits if buffer has been saved to file, warns if unsaved changes exist
Q - forcequits without warning
