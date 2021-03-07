Please note that I never say this will generate sentences, only words.

# Instruction
first have a dataset of text named training-dataset.txt
presumably pretty large  
then run `train.py` to create word/previous word pairs  
then run `counter.py` to count the appearences of those seperate pairs  
then run `produce.py` to actually generate something  
then open `result.txt` to see your text  
alternatively
run `cargo run` in `rust-version/produce-wp` to use the rust version, which is quicker, compiles natively, and gives better quality output (Python's RNG fucking sucks lmao)
